use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Copy, Clone, Debug)]
pub struct MaterialBounce {
    ray: Ray,
    attenuation: Color,
}

impl MaterialBounce {
    pub fn ray(self) -> Ray {
        self.ray
    }

    pub fn attenuation(self) -> Color {
        self.attenuation
    }
}

pub trait Material {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<MaterialBounce>;
}

pub struct EmptyMaterial;
impl Material for EmptyMaterial {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<MaterialBounce> {
        Some(MaterialBounce {
            ray,
            attenuation: Color::zero(),
        })
    }
}

pub struct LambertianMaterial {
    albedo: Color,
}
impl LambertianMaterial {
    pub fn new(albedo: Color) -> LambertianMaterial {
        LambertianMaterial { albedo }
    }
}
impl Material for LambertianMaterial {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<MaterialBounce> {
        let rec = rec.clone();
        let mut scatter_dir = rec.normal() + Vec3::rand_unit();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal();
        }

        let scattered = Ray::new(rec.point(), scatter_dir);

        Some(MaterialBounce {
            ray: scattered,
            attenuation: self.albedo,
        })
    }
}

pub struct MetalMaterial {
    albedo: Color,
    fuzz: f64,
}
impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f64) -> MetalMaterial {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        MetalMaterial { albedo, fuzz }
    }
}
impl Material for MetalMaterial {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<MaterialBounce> {
        let reflected = ray.direction().unit().reflect(rec.normal());
        let scattered = Ray::new(
            rec.point(),
            reflected + self.fuzz * Vec3::rand_in_unit_sphere(),
        );

        match scattered.direction().dot(rec.normal()) > 0.0 {
            true => Some(MaterialBounce {
                ray: scattered,
                attenuation: self.albedo,
            }),
            false => None,
        }
    }
}
