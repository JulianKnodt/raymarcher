extern crate rand;

use vector::vector::{Vector, gradient};
use vector::generate::generate;
use vector::ray::Ray;
use intersection::Intersection;
use material::Surface;

use std::f32;

#[derive(Debug)]
pub enum SDF {
  Sphere(Vector, f32, Surface), // location and radius
  Or(Box<SDF>, Box<SDF>), // an or of two SDFs
  And(Box<SDF>, Box<SDF>),
}

pub fn generate_sphere(around: Vector, nearness: f32, rad: f32, mat: Surface) -> SDF {
  SDF::Sphere(around + (generate() * (2.0 * rand::random::<f32>() - 1.0) * nearness),
    rand::random::<f32>() * rad, mat)
}

impl SDF {
  pub fn dist(&self, v: &Vector) -> f32 {
    match self {
     SDF::Sphere(loc, rad, _) => (loc - v).sqr_magn().sqrt() - rad,
     SDF::Or(a, b) => (*a).dist(v).min((*b).dist(v)),
     SDF::And(a, b) => (*a).dist(v).max((*b).dist(v)),
    }
  }
  pub fn get_surface(&self) -> Surface {
    match self {
      SDF::Sphere(_, _, s) => *s,

      // TODO think of something here
      SDF::Or(a, _) => (*a).get_surface(),
      SDF::And(a, _) => (*a).get_surface(),
    }
  }

  pub fn swap(self) -> SDF {
    match self {
      SDF::Or(a, b) => SDF::Or(b,a),
      SDF::And(a, b) => SDF::And(b,a),
      _ => self
    }
  }

  // Only returns option SDF if objects is empty.
  pub fn fold_into_or(objects: Vec<SDF>) -> Option<SDF> {
    let mut iter = objects.into_iter();
    match iter.next() {
      None => None,
      Some(sdf) =>
        Some(iter.fold(sdf, |res, next| SDF::Or(Box::new(next), Box::new(res.swap())))),
    }
  }

  pub fn unfold(self, mut into: Vec<SDF>) -> Vec<SDF> {
    match self {
      SDF::Or(a, b) | SDF::And(a, b) => a.unfold(b.unfold(into)),
      prim => {
        into.push(prim);
        into
      },
    }
  }
}


pub fn intersects(r: Ray, sdfs: &Vec<SDF>) -> Option<Intersection> {
  let mut min = f32::INFINITY;
  let mut nearest = None;
  for sdf in sdfs {
    match find_intersection(sdf, &r) {
      None => (),
      Some(t) if t < min && t > 0.0 => {
        min = t;
        nearest = Some(sdf);
      },
      Some(_) => (), // debugging branch
    }
  };
  match nearest {
    None => None,
    Some(ref s) => Some(Intersection{
      t: min,
      incident_ray: r.eps_shift(),
      normal: gradient(&|v| s.dist(v), &r.at(min)),
      surface: s.get_surface(),
    }),
  }
}

const MAX_DISTANCE: f32 = 100.0;
pub fn find_intersection(sdf: &SDF, r: &Ray) -> Option<f32> {
  let param_inter = |t| sdf.dist(&r.at(t));
  let mut t = 0.0;
  let mut delta_t = param_inter(t);
  let min_step = 0.0001;
  let epsilon = 0.00000001;
  while t < MAX_DISTANCE && delta_t > epsilon {
    t = t + delta_t.max(min_step);
    delta_t = param_inter(t);
  }
  if t > MAX_DISTANCE {
    None
  } else {
    Some(t)
  }
}


