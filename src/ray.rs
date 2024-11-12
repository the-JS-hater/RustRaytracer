use super::vec3::{dot, unit_vector, Color, Point, Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.dir * t
    }
    pub fn ray_color(&self) -> Color {
        let t = self.hit_sphere(
            &Point {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
        );

        if t > 0.0 {
            let n: Vec3 = unit_vector(
                self.at(t)
                    - Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: -1.0,
                    },
            );
            return Color {
                x: n.x + 1.0,
                y: n.y + 1.0,
                z: n.z + 1.0,
            } * 0.5;
        }

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

    pub fn hit_sphere(&self, center: &Point, radius: f64) -> f64 {
        let oc: Vec3 = *center - self.origin;
        let a: f64 = dot(&self.dir, &self.dir);
        let b: f64 = -2.0 * dot(&self.dir, &oc);
        let c: f64 = dot(&oc, &oc) - radius * radius;
        let discriminant: f64 = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a);
        }
    }
}
