use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub type Pixel = (u8, u8, u8);

pub struct ImageWriter {
    width: usize,
    height: usize,
    image_data: Vec<Pixel>,
}

impl ImageWriter {
    pub fn create(width: usize, height: usize) -> Self {
        let mut image_data = Vec::new();
        image_data.resize(width * height, (0, 0, 0));
        Self {
            width,
            height,
            image_data,
        }
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, pixel: Pixel) -> () {
        self.image_data[row * self.width + col] = pixel;
    }

    pub fn write_image(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut writer = BufWriter::new(File::create(path)?);

        writer.write("P3".as_bytes())?;
        writer.write("\n".as_bytes())?;
        writer.write(self.width.to_string().as_bytes())?;
        writer.write(" ".as_bytes())?;
        writer.write(self.height.to_string().as_bytes())?;
        writer.write("\n".as_bytes())?;
        writer.write("255".as_bytes())?;
        writer.write("\n".as_bytes())?;
        for pixel in &self.image_data {
            write!(&mut writer, "{} {} {}\n", pixel.0, pixel.1, pixel.2)?;
        }

        writer.flush()?;

        Ok(())
    }
}
