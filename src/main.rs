mod camera;
mod color;
mod hittable;
mod material;
mod math;
mod ray;
mod vec3;

use camera::Camera;
use color::as_ppm;
use hittable::{Hittable, HittableVec, Sphere};
use material::{LambertianMaterial, MetalMaterial};
use math::rand_double;
use ray::Ray;
use std::rc::Rc;
use vec3::{Color, Point3, Vec3};

const INFINITY: f64 = f64::MAX;

// Image constants
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    let mut world = HittableVec::new();

    let mat_ground = LambertianMaterial::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = LambertianMaterial::new(Color::new(0.7, 0.3, 0.3));
    let mat_left = MetalMaterial::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let mat_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(mat_ground),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(mat_center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(mat_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(mat_right),
    )));

    let world: Box<dyn Hittable> = Box::from(world);

    let cam = Camera::default();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanline: {} / {}", IMAGE_HEIGHT - y, IMAGE_HEIGHT);

        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + rand_double(0.0, 1.0)) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (y as f64 + rand_double(0.0, 1.0)) / (IMAGE_HEIGHT as f64 - 1.0);

                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(ray, &world, 0);
            }

            println!("{}", as_ppm(pixel_color, SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Done!");
}

const MAX_DEPTH: u32 = 50;
const WHITE: Color = Color::new(1.0, 1.0, 1.0);
fn ray_color(ray: Ray, world: &Box<dyn Hittable>, depth: u32) -> Color {
    if depth > MAX_DEPTH {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001..INFINITY) {
        Some(rec) => match rec.material().scatter(ray, rec) {
            Some(bounce) => bounce.attenuation() * ray_color(bounce.ray(), world, depth + 1),
            None => Color::new(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction = ray.direction().unit();
            let t = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - t) * WHITE + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}
