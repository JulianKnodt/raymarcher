use intersection::Intersection;
use vector::vector::Vector;
use vector::generate::generate;
use vector::ray::Ray;
use sdf::{SDF, find_intersection, intersects};
use std::f32;

pub enum Light {
  Point(Vector, Vector, f32) // Position, color, intensity
}

impl Light {
  pub fn biradiance(&self, to: &Vector) -> f32 {
    match self {
    Light::Point(loc, _, intensity) =>
        intensity / (4.0 * f32::consts::PI * (loc - &to).sqr_magn()),
    }
  }

  pub fn generate() -> Light {
    Light::Point(generate() * 20.0, Vector::new(1.0, 1.0, 1.0), 5000.0)
  }

  pub fn position(&self) -> Vector {
    match self {
      Light::Point(loc, _, _) => *loc,
    }
  }
}


const MAX_RECUR_DEPTH: i32 = 3;
pub fn illum_out(i: &Intersection, objects: &Vec<SDF>, lights: &Vec<Light>, recur: i32)
  -> Vector {
  let inter_pt = i.contact_point();

  lights.iter().fold(i.surface.emitted(), |acc, light| {
      if visible(inter_pt, light.position(), objects) {
        let incoming_dir = (inter_pt - light.position()).normalized();
        let bi = light.biradiance(&inter_pt);
        let (color, reflections) = i.surface.finite_sd(&i.incident_ray, &i.surface_normal());
        let comp = color * bi * i.normal.dot(&incoming_dir).abs();
        match reflections {
          None => (acc + comp) * (0.5),
          _ if recur >= MAX_RECUR_DEPTH => (acc + comp) * (0.5),
          Some(bounces) => {
            let sub = bounces.iter().filter_map(|b| intersects(b.eps_shift(), objects))
              .map(|sub_i| illum_out(&sub_i, objects, lights, recur + 1))
              .fold(Vector::new(0.0, 0.0, 0.0), |acc, n| acc + n);
            (sub + acc + comp) * (1.0 / ((2 +  bounces.len()) as f32))
          },
        }
      } else {
        acc
      }
    })
}

fn visible(to: Vector, from: Vector, objects: &Vec<SDF>) -> bool {
  // the subtraction is important because then it would might intersect the sdf it originally
  // hit

  let ray = Ray::from(from, to).eps_shift();
  let dist = (ray.position - to).sqr_magn().sqrt() - 0.001;

  objects.iter().find(|sdf| {
    match find_intersection(sdf, &ray) {
      None => false,
      Some(t) => t <= dist,
    }
  }).is_none()
}
