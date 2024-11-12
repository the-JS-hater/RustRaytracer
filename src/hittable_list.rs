use super::hit_record::{HitRecord, Hittable};
use super::ray::Ray;
use super::sphere::Sphere;
pub struct HittableList {
    pub spheres: Vec<Sphere>,
}

impl HittableList {
    pub fn add(&mut self, obj: Sphere) {
        self.spheres.push(obj)
    }

    pub fn clear(&mut self) {
        self.spheres = Vec::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.spheres {
            if obj.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList {
            spheres: Vec::new(),
        }
    }
}