use super::hit_record::{HitRecord, Hittable};
use super::interval::Interval;
use super::ray::Ray;
use super::vec3::{unit_vector, Color, Point, Vec3};
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
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let ray = Ray {
                    origin: self.center,
                    dir: ray_direction,
                };

                let pixel_color: Color = ray_color(&ray, world);

                let pixel: String = pixel_color.write_color();
                file.write_all(pixel.as_bytes())?;
            }
        }

        println!("Scan DONE!");
        Ok(())
    }
}

pub fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut temp_rec = HitRecord::default();
    if world.hit(
        ray,
        &Interval {
            min: 0.0,
            max: std::f64::INFINITY,
        },
        &mut temp_rec,
    ) {
        return (temp_rec.normal
            + Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            })
            * 0.5;
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
    };
}
