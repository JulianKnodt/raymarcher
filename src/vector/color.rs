extern crate image;

use vector::vector::Vector;

impl Vector {
  pub fn to_color(&self) -> image::Rgb<u8> {
    image::Rgb([(self.x * 255.0) as u8,
      (self.y * 255.0) as u8,
      (self.z * 255.0) as u8])
  }
}
