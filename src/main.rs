use std::io::{self};

use engine3d::vec3::{Point3};
use engine3d::hittable_list::HittableList;
use engine3d::sphere::Sphere;
use engine3d::camera::Camera;

fn main() -> io::Result<()> {

    // World
    let mut world : HittableList = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam : Camera = Camera::new(16.0 / 9.0, 400);
    let _ = cam.render(&world);

    Ok(())
}