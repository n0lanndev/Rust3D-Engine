use std::io::{self, Write};
mod vec3;
use vec3::Vec3;

fn main() -> io::Result<()> {
    let image_width: i64 = 256;
    let image_heigth: i64 = 256;

    let mut out = io::BufWriter::new(io::stdout());
    let mut clog = io::BufWriter::new(io::stderr());

    println!("P3\n{} {}\n255", image_width, image_heigth);

    for j in 0..image_heigth {
        let _ = writeln!(clog, "\rScanlines remaining: {}", image_heigth-j);
        clog.flush()?;
        for i in 0..image_width{
            let r: f64 = i as f64 / (image_width as f64 - 1.0);
            let g: f64 = j as f64 / (image_heigth as f64 - 1.0);
            let b: f64 = 0.0;

            let ir: i64 = (255.999 * r) as i64; 
            let ig: i64 = (255.999 * g) as i64;
            let ib: i64 = (255.999 * b) as i64;

            let _ = writeln!(out, "{} {} {}" , ir, ig, ib)?;
        }
    }

    let _ = writeln!(clog, "\nDone.");

    Ok(())
}
