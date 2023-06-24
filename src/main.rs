mod image_writer;
mod ray;
mod triangle;
mod v3;

use std::path::Path;

use image_writer::ImageWriter;

fn main() -> Result<(), std::io::Error> {
    let mut w = ImageWriter::create(5, 3);
    w.set_pixel(2, 1, (255, 255, 255));
    w.write_image(Path::new("./output.pbm"))
}
