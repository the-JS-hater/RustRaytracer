use super::vec3::{unit_vector, Color, Point, Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.dir * t
    }
    pub fn ray_color(&self) -> Color {
        let unit_direction: Vec3 = unit_vector(self.dir);
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
            } * a;
    }
}
