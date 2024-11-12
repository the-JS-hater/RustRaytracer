use super::hit_record::{HitRecord, Hittable};
use super::interval::Interval;
use super::ray::Ray;
use super::vec3::{dot, Point, Vec3};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_interval: &Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - ray.origin;
        let a: f64 = ray.dir.length_squared();
        let h: f64 = dot(&ray.dir, &oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root: f64 = (h - sqrtd) / a;
        if !t_interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t_interval.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        return true;
    }
}
