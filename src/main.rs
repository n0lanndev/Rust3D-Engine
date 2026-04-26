use std::io::{self, Write};
mod vec3;
use vec3::Vec3;
use vec3::Point3;
use vec3::unit_vector;
use vec3::dot;

mod color;
use color::write_color;
use color::Color;

mod ray;
use ray::Ray;

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool
{
    let oc: Vec3 = center - *ray.origin();
    let a: f64 = dot(*ray.direction(),*ray.direction());
    let b: f64 = -2.0 * dot(*ray.direction(), oc);
    let c: f64 = dot(oc, oc) - radius*radius;
    let discriminant: f64 = b*b - 4.0 * a * c;

    discriminant >= 0.0

}

fn ray_color(r : Ray) -> Color{
    if hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, &r)
    {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_vector : Vec3 = unit_vector(*r.direction());
    let a : f64 = 0.5 * (unit_vector.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {

    // Image
    
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i64 = 400;

    // Calculer l'image heigth, et s'assurer que c'est au moins 1
    
    let mut image_heigth: i64 = (image_width as f64 / aspect_ratio) as i64;
    image_heigth = if image_heigth < 1 { 1 } else { image_heigth };

    // Camera
    
    let focus_lenght : f64 = 1.0;
    let viewport_height : f64 = 2.0;
    let viewport_width : f64 = viewport_height * (image_width as f64 / image_heigth as f64);
    let camera_center : Point3 = Point3::default();

    // Vecteur viewport

    let viewport_u : Vec3 = Vec3::new(viewport_width, 0.0 , 0.0);
    let viewport_v : Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u : Vec3 = viewport_u / image_width as f64;
    let pixel_delta_v : Vec3 = viewport_v / image_heigth as f64;

    let viewport_upper_left : Point3 = camera_center - Vec3::new(0.0, 0.0, focus_lenght) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc : Point3 = viewport_upper_left + 0.5 *(pixel_delta_u + pixel_delta_v);

    let mut out = io::BufWriter::new(io::stdout());
    let mut clog = io::BufWriter::new(io::stderr());

    println!("P3\n{} {}\n255", image_width, image_heigth);

    for j in 0..image_heigth {
        let _ = writeln!(clog, "\rScanlines remaining: {}", image_heigth - j);
        clog.flush()?;
        for i in 0..image_width { 
            let pixel_center : Point3 = pixel00_loc + (i as f64 * pixel_delta_u + j as f64 * pixel_delta_v);
            let ray_direction : Vec3 = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction);

            let pixel_color : Color = ray_color(r);
            write_color(&mut out, &pixel_color); 
        }
    }

    let _ = writeln!(clog, "\nDone.");

    Ok(())
}
