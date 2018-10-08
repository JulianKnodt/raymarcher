use vector::vector::Vector;
use sdf::SDF;

pub struct Intersection<'a> {
  pub t: f32,
  pub normal: Vector,
  pub intersected: &'a SDF,
}

