use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::vec3::dot;

use crate::hittable::Hittable;
use crate::hittable::HitRecord;

use crate::ray::Ray;

pub struct Sphere{
    center: Point3,
    radius: f64,
}

impl Sphere{
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere{
            center: center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, r:&Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool{
        let oc: Vec3 = self.center - *r.origin();
        let a: f64 = r.direction().length_squared();
        let h: f64 = dot(*r.direction(), oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = h*h - a * c;

        if discriminant < 0.0
        {
            return false;
        }

        let sqrtd: f64 = discriminant.sqrt();

        let mut root: f64 = (h - sqrtd) / a;

        if root <= ray_tmin || ray_tmax <= root
        {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root
            {
                return false;
            }

        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal:Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}