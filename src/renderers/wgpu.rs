use std::{collections::HashMap, sync::mpsc, time::Duration};

use bytemuck::cast_slice;
use image::{ImageBuffer, Rgba};
use rand::random;
use util::{DeviceExt, TextureDataOrder};
use wgpu::*;

use crate::{camera::Camera, hittable::Hittable, render_parameters::RenderParameters, Float, Vec2};

pub fn render(
    camera: &Camera,
    world: &dyn Hittable,
    importants: &dyn Hittable,
    render_params: RenderParameters,
) -> Vec<Vec<crate::color::Color>> {
    pollster::block_on(render_async(camera, world, importants, render_params))
}

async fn render_async(
    camera: &Camera,
    world: &dyn Hittable,
    importants: &dyn Hittable,
    render_params: RenderParameters,
) -> Vec<Vec<crate::color::Color>> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::PRIMARY,
        ..Default::default()
    });
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .expect("Adapter should be available");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None, // Trace path
        )
        .await
        .unwrap();

    let color_texture_width = if render_params.image_width % 256 == 0 {
        render_params.image_width as u32
    } else {
        render_params.image_width as u32 + 256 - (render_params.image_width as u32 % 256)
    };

    let color_texture_size = wgpu::Extent3d {
        width: color_texture_width,
        height: render_params.image_height as u32,
        depth_or_array_layers: 1,
    };

    let ray_texture_size = wgpu::Extent3d {
        width: render_params.image_width as u32,
        height: render_params.image_height as u32,
        depth_or_array_layers: 1,
    };

    let color_buffer_desc = wgpu::BufferDescriptor {
        size: (render_params.image_width * render_params.image_height * 16) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };

    let color_buffer = device.create_buffer(&color_buffer_desc);

    let color_texture = device.create_texture(&wgpu::TextureDescriptor {
        // All textures are stored as 3D, we represent our 2D texture
        // by setting depth to 1.
        size: color_texture_size,
        mip_level_count: 1, // We'll talk about this a little later
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        // Most images are stored using sRGB, so we need to reflect that here.
        format: wgpu::TextureFormat::Rgba8Uint,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
        label: Some("render output"),
        view_formats: &[],
    });

    let ray_vec_len = render_params.image_width as usize * render_params.image_height as usize * 4;
    let mut ray_vec: Vec<Float> = vec![0.0; ray_vec_len];

    for i in (0..ray_vec_len).step_by(4) {
        let x = (i / 4) % render_params.image_width as usize;
        let y = (i / 4) / render_params.image_width as usize;
        let ray = generate_ray(camera, (x as i32, y as i32));
        let ray_slice = [ray.direction.x, ray.direction.y, ray.direction.z, 0.0];
        for j in 0..4 {
            ray_vec[i + j] = ray_slice[j];
        }
    }

    let ray_texture = device.create_texture_with_data(
        &queue,
        &wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: ray_texture_size,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB, so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::STORAGE_BINDING,
            label: Some("ray output"),
            view_formats: &[],
        },
        TextureDataOrder::LayerMajor,
        bytemuck::cast_slice(&ray_vec.as_slice()),
    );

    let sphere_array: [Float; 8] = [0.0, 0.0, 50.0, 25.0, 5.0, 5.0, 15.0, 5.0];

    let sphere_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&sphere_array),
        usage: BufferUsages::STORAGE,
    });

    let ray_color_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("ray color"),
        source: ShaderSource::Wgsl(include_str!("ray_color.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture {
                    access: StorageTextureAccess::WriteOnly,
                    format: TextureFormat::Rgba8Uint,
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture {
                    access: StorageTextureAccess::ReadOnly,
                    format: TextureFormat::Rgba32Float,
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&color_texture.create_view(
                    &TextureViewDescriptor {
                        label: None,
                        format: None,
                        dimension: Some(TextureViewDimension::D2),
                        aspect: TextureAspect::All,
                        base_mip_level: 0,
                        mip_level_count: None,
                        base_array_layer: 0,
                        array_layer_count: None,
                    },
                )),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&ray_texture.create_view(
                    &TextureViewDescriptor {
                        label: Some("ray texture bind group"),
                        format: None,
                        dimension: Some(TextureViewDimension::D2),
                        aspect: TextureAspect::All,
                        base_mip_level: 0,
                        mip_level_count: None,
                        base_array_layer: 0,
                        array_layer_count: None,
                    },
                )),
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &sphere_buffer,
                    size: None,
                    offset: 0,
                }),
            },
        ],
    });

    let entities_array: [u32; 4] = [0, 0, 1, 0];

    let entities_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
        label: None,
        contents: cast_slice(&entities_array),
        usage: BufferUsages::STORAGE,
    });

    let entities_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let entities_bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &entities_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: &entities_buffer,
                offset: 0,
                size: None,
            }),
        }],
    });

    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout, &entities_bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        entry_point: Some("compute_main"),
        cache: None,
        compilation_options: PipelineCompilationOptions {
            constants: &HashMap::new(),
            zero_initialize_workgroup_memory: false,
        },
        module: &ray_color_module,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.set_bind_group(1, &entities_bind_group, &[]);
        compute_pass.dispatch_workgroups(color_texture_size.width, color_texture_size.height, 1);
    }
    encoder.copy_texture_to_buffer(
        ImageCopyTexture {
            texture: &color_texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        ImageCopyBuffer {
            buffer: &color_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(color_texture_size.width * 4),
                rows_per_image: None,
            },
        },
        color_texture_size,
    );
    queue.submit(std::iter::once(encoder.finish()));

    let black = crate::color::Color::new(0.0, 0.0, 0.0);
    let mut result: Vec<Vec<crate::color::Color>> =
        vec![vec![black; render_params.image_width as usize]; render_params.image_height as usize];
    {
        let (tx, rx) = mpsc::channel();
        let buffer_slice = color_buffer.slice(..);

        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.recv_timeout(Duration::from_secs(5)).unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();

        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
            render_params.image_width as u32,
            render_params.image_height as u32,
            data,
        )
        .unwrap();
        let mut buffer_iter = buffer.iter();
        for y in 0..render_params.image_height as usize {
            for x in 0..render_params.image_width as usize {
                result[y][x] = crate::color::Color::new(
                    *buffer_iter.next().unwrap() as Float,
                    *buffer_iter.next().unwrap() as Float,
                    *buffer_iter.next().unwrap() as Float,
                );
                let _a = buffer_iter.next();
            }
        }
    }
    color_buffer.unmap();

    return result;
}

fn generate_ray(camera: &Camera, (x, y): (i32, i32)) -> crate::ray::Ray {
    let pixel_center = camera.pixel00_loc
        + (x as Float * camera.pixel_delta_u)
        + (y as Float * camera.pixel_delta_v);

    let pixel_sample = pixel_center + pixel_sample_square(camera);

    let ray_origin = if camera.defocus_angle <= 0.0 {
        camera.center
    } else {
        defocus_disk_sample(camera)
    };

    let ray_direction = pixel_sample - ray_origin;

    return crate::ray::Ray::new(ray_origin, ray_direction.normalize());
}

fn pixel_sample_square(camera: &Camera) -> crate::Vec3 {
    let px: Float = -0.5 + random::<Float>();
    let py: Float = -0.5 + random::<Float>();

    return (camera.pixel_delta_u * px) + (camera.pixel_delta_v * py);
}

fn defocus_disk_sample(camera: &Camera) -> crate::Vec3 {
    let p = Vec2::new(0.5, 0.5);
    return camera.center + (p.x * camera.defocus_disk_u) + (p.y * camera.defocus_disk_v);
}
