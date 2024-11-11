use std::fs::File;
use std::io::Write;
mod vec3;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.ppm")?;

    // Image
    const IMAGE_HEIGHT: u16 = 256;
    const IMAGE_WIDTH: u16 = 256;

    // Render
    let formatted_str = format!("P3\n{} {}\n255\n", IMAGE_HEIGHT, IMAGE_WIDTH);
    file.write_all(formatted_str.as_bytes())?;

    for i in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining {}", IMAGE_HEIGHT - i);

        for j in 0..IMAGE_WIDTH {
            let pixel_color: vec3::Color = vec3::Vec3 {
                x: (i as f64 / IMAGE_WIDTH as f64 * 255.0),
                y: (j as f64 / IMAGE_HEIGHT as f64 * 255.0),
                z: 0.0,
            };

            let pixel: String = pixel_color.write_color();
            file.write_all(pixel.as_bytes())?;
        }
    }

    println!("Scan DONE!");
    Ok(())
}
