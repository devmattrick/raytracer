use std::{ops::Range, rc::Rc};

use crate::{
    material::{EmptyMaterial, Material},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    material: Rc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            point: Point3::zero(),
            normal: Vec3::zero(),
            material: Rc::new(EmptyMaterial),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn point(&self) -> Point3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    fn face_normal(ray: Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.direction().dot(outward_normal) < 0.0;

        (
            front_face,
            match front_face {
                true => outward_normal,
                false => -outward_normal,
            },
        )
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t: Range<f64>) -> Option<HitRecord>;
}

pub struct HittableVec {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableVec {
    pub fn new() -> HittableVec {
        HittableVec {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableVec {
    fn hit(&self, ray: Ray, t: Range<f64>) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::default();
        let mut hit = false;
        let mut closest_so_far = t.end;

        for object in &self.objects {
            match object.hit(ray, t.start..closest_so_far) {
                Some(rec) => {
                    temp_rec = rec;
                    hit = true;
                    closest_so_far = temp_rec.t;
                }
                _ => {}
            }
        }

        if !hit {
            return None;
        }

        Some(temp_rec)
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t: Range<f64>) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        let a = ray.direction().len_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.len_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        if !t.contains(&root) {
            let root = (-half_b - sqrtd) / a;

            if !t.contains(&root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let material = self.material.clone();
        let normal = (point - self.center) / self.radius;
        let (front_face, normal) = HitRecord::face_normal(ray, normal);

        Some(HitRecord {
            t,
            point,
            material,
            normal,
            front_face,
        })
    }
}
