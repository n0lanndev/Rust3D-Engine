use std::io::{self, Write};

use crate::vec3::{Vec3, Point3, unit_vector};
use crate::color::{Color, write_color};
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval};
use crate::utility::random_double;

pub struct Camera
{
    pub aspect_ratio : f64,
    pub image_width : i64,
    pub samples_per_pixel : i64,
    pixel_samples_scale : f64,
    image_heigth : i64,
    center : Point3,
    pixel00_loc : Point3,
    pixel_delta_u : Vec3,
    pixel_delta_v: Vec3,
}

impl Camera
{
    pub fn new(aspect_ratio: f64, image_width: i64) -> Camera{
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel : 10,
            pixel_samples_scale : 0.0,
            image_heigth : 0,
            center : Point3::default(),
            pixel00_loc : Point3::default(),
            pixel_delta_u : Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }

    fn initialize(&mut self)
    {
        self.image_heigth = (self.image_width as f64 / self.aspect_ratio) as i64;
        self.image_heigth = if self.image_heigth < 1 { 1 } else { self.image_heigth };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Point3::default();
        
        let focus_lenght : f64 = 1.0;
        let viewport_height : f64 = 2.0;
        let viewport_width : f64 = viewport_height * (self.image_width as f64 / self.image_heigth as f64);

        let viewport_u : Vec3 = Vec3::new(viewport_width, 0.0 , 0.0);
        let viewport_v : Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_heigth as f64;

        let viewport_upper_left : Point3 = self.center - Vec3::new(0.0, 0.0, focus_lenght) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 *(self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render<T: Hittable>(&mut self, world : &T) -> io::Result<()>{

        self.initialize();

        let mut out = io::BufWriter::new(io::stdout());
        let mut clog = io::BufWriter::new(io::stderr());

        let _ = writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_heigth);

        for j in 0..self.image_heigth { 
            let _ = writeln!(clog, "\rScanlines remaining: {}", self.image_heigth - j);
            clog.flush()?;
            for i in 0..self.image_width { 
                let mut pixel_color : Color = Color::default();
                for sample in 0..self.samples_per_pixel{
                    let r : Ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(r, world);
                }

                write_color(&mut out, &(self.pixel_samples_scale * pixel_color)); 
            }
        }

        let _ = writeln!(clog, "\nDone.");
        Ok(())
    }
    
    fn get_ray(&self, i: i64, j: i64) -> Ray{
        let offset : Vec3 = self.sample_square();
        let pixel_sample : Point3 = self.pixel00_loc 
            + ((i as f64 + offset.x) * self.pixel_delta_u) 
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_direction : Vec3 = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn sample_square(&self) -> Vec3{
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color<T: Hittable>(r : Ray, world : &T) -> Color{

        let mut rec : HitRecord = HitRecord::default();
        if world.hit(&r, Interval::new(0.0, f64::INFINITY), &mut rec){
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        } 

        let unit_vector : Vec3 = unit_vector(*r.direction());
        let a : f64 = 0.5 * (unit_vector.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}