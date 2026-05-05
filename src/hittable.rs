use crate::vec3::{Vec3, Point3};
use crate::vec3::dot;
use crate::ray::Ray;

#[derive(Default, Clone)]
pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord{
    pub fn set_face_normal(&mut self, r:&Ray, outward_normal: Vec3){
        self.front_face = dot(*r.direction(), outward_normal) < 0.0;

        if self.front_face 
        {
            self.normal = outward_normal;
        }
        else
        {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable{
    fn hit(&self, r:&Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}