mod image_writer;
mod ray;
mod triangle;
mod v3;

use std::path::Path;

use image_writer::{ImageWriter, Pixel};
use ray::Ray;
use triangle::Triangle;
use v3::V3;

fn main() -> Result<(), std::io::Error> {
    let width = 500;
    let height = 300;
    let mut w = ImageWriter::create(width, height);

    let camera = V3::create(0.5, 0.5, 0.0);

    let objects = [
        Triangle::create_color(
            V3::create(-1.0, 0.0, 2.0),
            V3::create(1.0, 0.0, 2.0),
            V3::create(0.0, -1.0, 3.0),
            (255, 0, 0),
        ),
        Triangle::create_color(
            V3::create(1.0, 0.0, 2.0),
            V3::create(1.0, 0.0, 4.0),
            V3::create(0.0, -1.0, 3.0),
            (0, 255, 0),
        ),
        Triangle::create_color(
            V3::create(-1.0, 0.0, 2.0),
            V3::create(-1.0, 0.0, 4.0),
            V3::create(0.0, -1.0, 3.0),
            (0, 0, 255),
        ),
        Triangle::create_color(
            V3::create(-1.0, 0.0, 4.0),
            V3::create(1.0, 0.0, 4.0),
            V3::create(0.0, -1.0, 3.0),
            (255, 0, 255),
        ),
    ];

    let pixel_size = 0.004;
    for r in 0..height {
        for c in 0..width {
            let px = pixel_size * (c as f64 - (width as f64) / 2.0);
            let py = pixel_size * (r as f64 - (height as f64) / 2.0);
            let ray = Ray::create(camera, V3::create(px, py, 1.0));

            let closest_intersection = objects
                .iter()
                .filter_map(|obj| obj.intersect(&ray).map(|v| (v.length(), obj.color())))
                .min_by(|(d_a, _), (d_b, _)| d_a.partial_cmp(d_b).unwrap());
            if let Some((distance, color)) = closest_intersection {
                let scale = 1.0 / distance;
                w.set_pixel(r, c, scale_pixel(color, scale))
            }
        }
    }

    w.write_image(Path::new("./output.pbm"))
}

fn scale_pixel(pixel: Pixel, scale: f64) -> Pixel {
    (
        ((pixel.0 as f64) * scale) as u8,
        ((pixel.1 as f64) * scale) as u8,
        ((pixel.2 as f64) * scale) as u8,
    )
}
