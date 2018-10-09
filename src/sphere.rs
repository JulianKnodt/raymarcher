use vector::vector::Vector;
use sdf::SDF;

struct Sphere {
  location: Vector,
  sqr_radius: f32,
}

impl SDF for Sphere {
  fn dist(&self, v: Vector) -> f32 {
    (self.location - v).sqr_magn() - self.sqr_radius
  }
}
