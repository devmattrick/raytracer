mod camera;
mod color;
mod hittable;
mod material;
mod math;
mod ray;
mod vec3;

use camera::Camera;
use chrono::{DateTime, Utc};
use color::as_ppm;
use hittable::{Hittable, HittableVec, Sphere};
use material::{DielectricMaterial, LambertianMaterial, Material, MetalMaterial};
use math::rand_double;
use ray::Ray;
use std::{rc::Rc};
use vec3::{Color, Point3};

const INFINITY: f64 = f64::MAX;

// Image constants
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    let start: DateTime<Utc> = Utc::now();
    eprintln!("Starting render at {}", start);

    let world: Box<dyn Hittable> = Box::from(random_scene());

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        focus_dist,
    );

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

    let end: DateTime<Utc> = Utc::now();
    let duration = end.signed_duration_since(start);
    eprintln!("Finished render at {} in {}.", end, duration);
}

fn random_scene() -> HittableVec {
    let mut world = HittableVec::new();

    let mat_ground = LambertianMaterial::new(Color::new(1.0, 1.0, 1.0));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        Rc::new(mat_ground),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * rand_double(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * rand_double(0.0, 1.0),
            );

            let rand_mat = rand_double(0.0, 1.0);
            let mat: Rc<dyn Material> = if rand_mat < 0.8 {
                let albedo = Color::rand(0.0, 1.0) * Color::rand(0.0, 1.0);
                Rc::new(LambertianMaterial::new(albedo))
            } else if rand_mat < 0.95 {
                let albedo = Color::rand(0.5, 1.0);
                let fuzz = rand_double(0.0, 0.5);
                Rc::new(MetalMaterial::new(albedo, fuzz))
            } else {
                let refraction_index = rand_double(1.3, 2.7);
                Rc::new(DielectricMaterial::new(refraction_index))
            };

            world.add(Box::new(Sphere::new(center, 0.2, mat)));
        }
    }

    let mat_1 = Rc::new(DielectricMaterial::new(1.5));
    let mat_2 = Rc::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    let mat_3 = Rc::new(MetalMaterial::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        mat_1,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat_2,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        mat_3,
    )));

    world
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
