use std::{collections::HashMap, hash::Hash, num::NonZero, sync::mpsc, time::Duration};

use image::{ImageBuffer, Rgba};
use wgpu::*;

use crate::{camera::Camera, hittable::Hittable, render_parameters::RenderParameters};

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

    let texture_size = wgpu::Extent3d {
        width: render_params.image_width as u32,
        height: render_params.image_height as u32,
        depth_or_array_layers: 1,
    };

    let output_buffer_desc = wgpu::BufferDescriptor {
        size: (render_params.image_width * render_params.image_height * 16) as u64,
        usage: wgpu::BufferUsages::COPY_DST
            // this tells wpgu that we want to read this buffer from the cpu
            | wgpu::BufferUsages::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };

    let output_buffer = device.create_buffer(&output_buffer_desc);

    let rendered_texture = device.create_texture(&wgpu::TextureDescriptor {
        // All textures are stored as 3D, we represent our 2D texture
        // by setting depth to 1.
        size: texture_size,
        mip_level_count: 1, // We'll talk about this a little later
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        // Most images are stored using sRGB, so we need to reflect that here.
        format: wgpu::TextureFormat::Rgba8Uint,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
        label: Some("render output"),
        view_formats: &[],
    });

    let ray_color_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("ray color"),
        source: ShaderSource::Wgsl(include_str!("ray_color.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::WriteOnly,
                format: TextureFormat::Rgba8Uint,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        }],
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&rendered_texture.create_view(
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
        }],
    });

    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
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
        compute_pass.dispatch_workgroups(texture_size.width, texture_size.height, 1);
    }
    encoder.copy_texture_to_buffer(
        ImageCopyTexture {
            texture: &rendered_texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        ImageCopyBuffer {
            buffer: &output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(texture_size.width * 4),
                rows_per_image: None,
            },
        },
        texture_size,
    );
    queue.submit(std::iter::once(encoder.finish()));

    {
        let (tx, rx) = mpsc::channel();
        let buffer_slice = output_buffer.slice(..);

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

        buffer.save("gpu.png").unwrap();
    }
    output_buffer.unmap();

    vec![]
}
