use vector::vector::{Vector, Origin};
use intersection::Intersection;

use std::f32;

// Trait for signed distance fields
pub trait SDF {
  // Returns distance of v from self
  fn dist(&self, v: &Vector) -> f32;
}

fn Intersects<'a>(v: &Vector, sdfs: &Vec<&'a SDF>) -> Option<Intersection<'a>> {
  let mut min = f32::INFINITY;
  let mut nearest = None;
  for sdf in sdfs {
    let t = find_intersection(sdf, v);
    if t < min {
      min = t;
      nearest = Some(sdf);
    }
  }
  match nearest {
    None => None,
    Some(&v) => Some(Intersection{
      t: min,
      normal: Origin(), //TODO
      intersected: v,
    }),
  }
}

fn find_intersection(sdf: &SDF, v: &Vector) -> f32 {
  // want to find a zero to the function as fast as possible
  panic!("TODO IMPLEMENT")
}

