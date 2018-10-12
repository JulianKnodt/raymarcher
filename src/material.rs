extern crate rand;

use vector::ray::Ray;
use vector::vector::Vector;
use vector::generate::generate;
use std::f32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Surface {
  Empty,
  Metal(Vector),
  Lambertian(Vector, f32),
}

impl Surface {
  pub fn emitted(&self) -> Vector {
    match self {
      Surface::Metal(c) | Surface::Lambertian(c, _) => c.clone(),
      Surface::Empty => Vector::new(0.0,0.0,0.0),
    }
  }

  pub fn finite_sd(&self, r: &Ray, s_n: &Ray) -> (Vector, Option<Vec<Ray>>) {
    match self {
      Surface::Empty => (Vector::new(1.0,1.0,1.0), None),
      Surface::Lambertian(color, albedo) => (color * (1.0/albedo), None),
      Surface::Metal(color) => {
        let reflected_direction = r.direction.reflect_across(&s_n.direction);
        let reflected = Ray{
          position: s_n.position,
          direction: reflected_direction.normalized(),
        };
        (color.clone(), Some(vec!(reflected)))
      },
    }
  }
}


pub fn random_surface() -> Surface {
  match rand::random::<u8>() {
    _ => Surface::Metal(generate()),
  }
}
