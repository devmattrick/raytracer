use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const VIEWPORT_HEIGHT: f64 = 2.0;
pub const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH: f64 = 1.0;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Point3,
    horizontal: Point3,
    vertical: Point3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(origin: Point3, horizontal: Point3, vertical: Point3) -> Camera {
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, FOCAL_LENGTH),
        }
    }

    pub fn default() -> Camera {
        Self::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0),
            Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0),
        )
    }

    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
