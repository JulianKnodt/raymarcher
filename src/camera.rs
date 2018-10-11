use vector::vector::Vector;
use vector::ray::Ray;

#[derive(Clone, Copy)]
pub struct Camera {
// TODO  position: Vector,
  pub z_near: f32,
  pub vert_fov: f32,
}

impl Camera {
  pub fn get_ray_to(&self, x: f32, y: f32, width: i32, height: i32) -> Ray {
    let side = -2.0 * (self.vert_fov/ 2.0).tan();
    let x = self.z_near * (x/(width as f32) - 0.5) * side * (width as f32)/(height as f32);
    let y = self.z_near * -(y/(height as f32) - 0.5) * side;
    let p = Vector{x, y, z: self.z_near};
    Ray{
      position: p,
      direction: p.clone().normalized(),
    }
  }

  pub fn
  get_rays_to(&self, x: f32, y: f32, width: i32, height: i32, parts: i32) -> Vec<Ray> {
    let partition_amt = 1.0/(parts as f32);
    (1..parts).map(|p| (p as f32) * partition_amt).flat_map(|dx| {
      (1..parts).map(|p| (p as f32) * partition_amt).map(move |dy|
        self.get_ray_to(x + dx, y + dy, width, height))
    }).collect()
  }
}
