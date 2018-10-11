use intersection::Intersection;
use vector::vector::Vector;
use vector::generate::generate;
use vector::ray::Ray;
use sdf::{SDF, find_intersection};
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
      Light::Point(pos, _, _) => *pos,
    }
  }
}


pub fn illum_out(i: &Intersection, objects: &Vec<SDF>, lights: &Vec<Light>) -> Vector {
  let inter_pt = i.contact_point();

  lights.iter()
    .fold(i.surface.emitted(), |acc, light| {
      if visible(light.position(), inter_pt, objects) {
        let incoming_dir = (inter_pt - light.position()).normalized();
        let bi = light.biradiance(&inter_pt);
        let color = i.surface.finite_sd(&i.incident_ray, &i.surface_normal());
        let comp = color * bi * i.normal.dot(&incoming_dir).abs();
        (acc + comp) * (0.5)
      } else {
        acc
      }
    })
}

fn visible(from: Vector, to: Vector, objects: &Vec<SDF>) -> bool {
  let dist = (from - to).sqr_magn().sqrt();
  let ray = Ray::from(from, to).eps_shift();
  objects.iter().find(|sdf| {
    match find_intersection(sdf, &ray) {
      None => false,
      Some(t) => t < dist,
    }
  }).is_none()
}
