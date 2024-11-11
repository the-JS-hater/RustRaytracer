use std::fs::File;
use std::io::Write;
mod ray;
use ray::Ray;
mod vec3;
use vec3::{Color, Point, Vec3};

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.ppm")?;
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let image_height: u32 = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
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

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        }
        - (viewport_u / 2.0)
        - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

    // Render
    let formatted_str = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write_all(formatted_str.as_bytes())?;

    for j in 0..image_height {
        println!("Scanlines remaining {}", image_height - j);

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray {
                origin: camera_center,
                dir: ray_direction,
            };
            let pixel_color = ray.ray_color();

            let pixel: String = pixel_color.write_color();
            file.write_all(pixel.as_bytes())?;
        }
    }

    println!("Scan DONE!");
    Ok(())
}
