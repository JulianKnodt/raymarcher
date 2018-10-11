extern crate rand;

use vector::vector::Vector;

pub fn generate() -> Vector {
  Vector{
    x: rand::random(),
    y: rand::random(),
    z: rand::random(),
  }
}
