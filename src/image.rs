extern crate image;

use vector::vector::Vector;
use std::io::Result;

pub fn image(img: Vec<Vec<Vector>>, filename: String) -> Result<()> {
  let width = img.len() as u32;
  let height = img[0].len() as u32;
  let mut imgbuf = image::ImageBuffer::new(width, height);
  for (y, row) in img.iter().enumerate() {
    for (x, px) in row.iter().enumerate() {
      imgbuf.put_pixel(x as u32, y as u32, image::Rgb([px.x as u8, px.y as u8, px.z as u8]));
    }
  }
  image::ImageRgb8(imgbuf).save(filename)?;
  Ok(())
}
