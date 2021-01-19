use crate::{
    hittable::HitRecord,
    math::rand_double,
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
    fn scatter(&self, ray: Ray, _: HitRecord) -> Option<MaterialBounce> {
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
    fn scatter(&self, _: Ray, rec: HitRecord) -> Option<MaterialBounce> {
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

#[derive(Clone)]
pub struct DielectricMaterial {
    refraction_index: f64,
}
impl DielectricMaterial {
    pub fn new(refraction_index: f64) -> DielectricMaterial {
        DielectricMaterial { refraction_index }
    }

    fn reflectance(&self, cosine: f64) -> f64 {
        let r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        let r0 = r0.powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
impl Material for DielectricMaterial {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<MaterialBounce> {
        let attenuation: Color = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = ray.direction().unit();
        let cos_theta = f64::min((-unit_dir).dot(rec.normal()), 1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || self.reflectance(cos_theta) > rand_double(0.0, 1.0) {
            unit_dir.reflect(rec.normal())
        } else {
            unit_dir.refract(rec.normal(), refraction_ratio)
        };

        Some(MaterialBounce {
            ray: Ray::new(rec.point(), direction),
            attenuation,
        })
    }
}
