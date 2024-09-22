use crate::{Float, PI};

use crate::Vec3;

use crate::{hittable::Hittable, onb::ONB, rand_vec3::random_cosine_direction};

pub trait PDF {
    fn value(&self, direction: &Vec3) -> Float;
    fn generate(&self) -> Vec3;
}

pub struct CosinePDF {
    uvw: ONB
}

impl CosinePDF {
    pub fn new(uvw: ONB) -> Self {
        Self {
            uvw
        }
    }
}


impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> Float {
        let cosine_theta = direction.normalize().dot(self.uvw.w());
        return (cosine_theta / PI).max(0.0);
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(&random_cosine_direction())
    }
}

pub struct HittablePDF<'a> {
    origin: Vec3,
    objects: &'a dyn Hittable,
}

impl<'a> HittablePDF<'a> {
    pub fn new(origin: Vec3, objects: &'a dyn Hittable) -> Self {
        Self { origin, objects }
    }
}

impl<'a> PDF for HittablePDF<'a> {
    fn value(&self, direction: &Vec3) -> Float {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random_vector_to_surface(&self.origin)
    }
}

pub struct MixturePDF<'a> {
    a: &'a dyn PDF,
    b: &'a dyn PDF,
}

impl<'a> MixturePDF<'a> {
    pub fn new(a: &'a dyn PDF, b: &'a dyn PDF) -> Self {
        Self {
            a, b
        }
    }
}

impl<'a> PDF for MixturePDF<'a> {
    fn generate(&self) -> Vec3 {
        let r: Float = rand::random();
        if r < 0.5 {
            self.a.generate()
        } else {
            self.b.generate()
        }
    }

    fn value(&self, direction: &Vec3) -> Float {
        // let weight = 1.0 / self.mixed.len() as Float;
        // self.mixed.iter().fold(0.0, |acc, m| {
        //     acc + weight * m.value(direction)
        // })
        self.a.value(direction) * 0.5 + self.b.value(direction) * 0.5
    }
}