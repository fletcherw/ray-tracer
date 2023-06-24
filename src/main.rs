mod image_writer;
mod ray;
mod triangle;
mod v3;

use std::path::Path;

use image_writer::ImageWriter;
use ray::Ray;
use triangle::Triangle;
use v3::V3;

fn main() -> Result<(), std::io::Error> {
    let width = 500;
    let height = 300;
    let mut w = ImageWriter::create(width, height);

    let camera = V3::create(0.0, 0.0, 0.0);

    let t = Triangle::create(
        V3::create(-1.0, 0.0, 2.0),
        V3::create(1.0, 0.0, 2.0),
        V3::create(0.0, -1.0, 5.0),
    );

    let pixel_size = 0.004;
    for r in 0..height {
        for c in 0..width {
            let px = pixel_size * (r as f64 - (height as f64) / 2.0);
            let py = pixel_size * (c as f64 - (width as f64) / 2.0);
            let ray = Ray::create(camera, V3::create(px, py, 1.0));
            if let Some(i) = t.intersect(&ray) {
                let distance = (i - camera).length();
                w.set_pixel(r, c, ((255.0 / f64::sqrt(distance)) as u8, 0, 0))
            }
        }
    }

    w.write_image(Path::new("./output.pbm"))
}
