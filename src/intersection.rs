use vector::vector::Vector;
use vector::ray::Ray;
use material::Surface;

pub struct Intersection {
  pub t: f32,
  pub incident_ray: Ray,
  pub normal: Vector,
  pub surface: Surface,
}

impl Intersection {
  pub fn contact_point(&self) -> Vector {
    self.incident_ray.at(self.t)
  }
  pub fn surface_normal(&self) -> Ray {
    Ray{
      position: self.contact_point(),
      direction: self.normal.normalized(),
    }.eps_shift()
  }
}
