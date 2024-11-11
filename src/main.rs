use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.ppm")?;

    // Image
    const IMAGE_HEIGHT: u16 = 256;
    const IMAGE_WIDTH: u16 = 256;

    // Render
    let formatted_str = format!("P3\n{} {}\n255\n", IMAGE_HEIGHT, IMAGE_WIDTH);
    file.write_all(formatted_str.as_bytes())?;

    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let r = (i as f32 / IMAGE_WIDTH as f32 * 255.0) as u8;
            let g = (j as f32 / IMAGE_HEIGHT as f32 * 255.0) as u8;
            let b = 0;

            let pixel = format!("{} {} {}\n", r, g, b);
            file.write_all(pixel.as_bytes())?;
        }
    }

    Ok(())
}
