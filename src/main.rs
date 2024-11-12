#![allow(dead_code)]

mod camera;
mod hit_record;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;
use camera::{initialize, Camera};
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point;

fn main() -> std::io::Result<()> {
    // World
    let world: &mut HittableList = &mut HittableList::default();

    world.add(Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    });

    world.add(Sphere {
        center: Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    });

    let cam: Camera = initialize();
    let _ = cam.render(world);

    Ok(())
}
