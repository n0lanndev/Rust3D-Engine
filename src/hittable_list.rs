use crate::hittable::Hittable;

use crate::ray::Ray;

use crate::hittable::HitRecord;

use crate::interval::Interval;

#[derive(Default)]
pub struct HittableList{
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList{
    pub fn new() -> HittableList{
        HittableList { objects: Vec::new()}
    }
    pub fn with_objet(object: Box<dyn Hittable>) -> HittableList{
        let mut list = Self::new();
        list.add(object);
        list
    }   

    pub fn clear(&mut self){
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>){
        self.objects.push(object);
    }
}

impl Hittable for HittableList{
    fn hit(&self, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool
    {
        let mut temp_rec : HitRecord = HitRecord::default();
        let mut hit_anything : bool = false;
        let mut closest_so_far : f64 = ray_t.max;

        for object in &self.objects{
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }
}