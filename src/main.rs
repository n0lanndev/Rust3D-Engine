use std::io::{self, Write};
mod vec3;
use vec3::Vec3;

mod color;
use color::write_color;
use color::Color;

fn main() -> io::Result<()> {
    let image_width: i64 = 256;
    let image_heigth: i64 = 256;

    let mut out = io::BufWriter::new(io::stdout());
    let mut clog = io::BufWriter::new(io::stderr());

    println!("P3\n{} {}\n255", image_width, image_heigth);

    for j in 0..image_heigth {
        let _ = writeln!(clog, "\rScanlines remaining: {}", image_heigth - j);
        clog.flush()?;
        for i in 0..image_width {
            let pixel_color:Color = Color{x: i as f64 / (image_width as f64-1.0), y: j as f64 / (image_heigth as f64 -1.0), z: 0.0};
            write_color(&mut out, &pixel_color); 
        }
    }

    let _ = writeln!(clog, "\nDone.");

    Ok(())
}
