use super::hit_record::{HitRecord, Hittable};
use super::interval::Interval;
use super::ray::Ray;
use super::utils::{rnd_float, rnd_float_range};
use super::vec3::{rand_unit_vector, random_on_hemisphere, unit_vector, Color, Point, Vec3};
use super::HittableList;
use std::fs::File;
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub center: Point,
    pub pixel00_loc: Point,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub samples_per_pixel: u32,
    pub pixel_samples_scale: f64,
    pub max_depth: u32,
}

impl Camera {
    pub fn render(&self, world: &HittableList) -> std::io::Result<()> {
        let mut file = File::create("output.ppm")?;
        // Render
        let formatted_str = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        file.write_all(formatted_str.as_bytes())?;

        for j in 0..self.image_height {
            println!("Scanlines remaining {}", self.image_height - j);

            for i in 0..self.image_width {
                let mut pixel_color: Color = Color {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                for _ in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(i, j);
                    pixel_color += ray_color(&ray, self.max_depth, world);
                }

                let pixel: String = (pixel_color * self.pixel_samples_scale).write_color();
                file.write_all(pixel.as_bytes())?;
            }
        }

        println!("Scan DONE!");
        Ok(())
    }
    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset: Vec3 = sample_square();
        let pixel_sample: Point = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x))
            + (self.pixel_delta_v * (j as f64 + offset.y));
        let ray_origin: Point = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray {
            origin: ray_origin,
            dir: ray_direction,
        };
    }
}

pub fn ray_color(ray: &Ray, max_depth: u32, world: &HittableList) -> Color {
    if max_depth == 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    let mut temp_rec = HitRecord::default();
    if world.hit(
        ray,
        &Interval {
            min: 0.001,
            max: std::f64::INFINITY,
        },
        &mut temp_rec,
    ) {
        let direction: Vec3 = temp_rec.normal + rand_unit_vector();
        let direction: Vec3 = random_on_hemisphere(&temp_rec.normal);
        return ray_color(
            &Ray {
                origin: temp_rec.p,
                dir: direction,
            },
            max_depth - 1,
            world,
        ) * 0.5;
    }

    let unit_direction: Vec3 = unit_vector(ray.dir);
    let a: f64 = 0.5 * (unit_direction.y + 1.0);
    return Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * (1.0 - a)
        + Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };
}

pub fn initialize() -> Camera {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let image_height: u32 = if image_height < 1 { 1 } else { image_height };
    let samples_per_pixel = 100;
    let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
    let max_depth = 50;
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let viewport_u = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };

    let viewport_v = Vec3 {
        x: 0.0,
        y: -viewport_height,
        z: 0.0,
    };

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        }
        - (viewport_u / 2.0)
        - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);
    return Camera {
        image_width,
        image_height,
        aspect_ratio,
        center: camera_center,
        pixel00_loc,
        pixel_delta_u,
        pixel_delta_v,
        samples_per_pixel,
        pixel_samples_scale,
        max_depth,
    };
}

pub fn sample_square() -> Vec3 {
    return Vec3 {
        x: rnd_float() - 0.5,
        y: rnd_float() - 0.5,
        z: 0.0,
    };
}
