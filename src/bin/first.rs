extern crate raymarcher;
extern crate image;
extern crate rand;

use raymarcher::camera::Camera;
use raymarcher::sdf::{SDF, intersects, generate_sphere};
use raymarcher::vector::vector::Vector;
use raymarcher::material::{random_surface};
use raymarcher::light::{Light, illum_out};

use image::{ImageBuffer};

fn main() {
  let width = 800;
  let height = 800;
  let cam = Camera{
    z_near: 1.0,
    vert_fov: 32.0,
  };
  let mut objects: Vec<SDF> = Vec::new();
  for _ in 0..25 {
    objects.push(generate_sphere(Vector::new(0.0, 0.0,30.0), 10.0, 1.0, random_surface()));
  }

  let mut lights: Vec<Light> = Vec::new();
  for _ in 0..5 {
    lights.push(Light::generate());
  }


  //let white = Vector{x: 1.0, y: 1.0, z: 1.0};
  let black = Vector{x: 0.0, y: 0.0, z: 0.0};

  ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
    let ray = cam.get_ray_to(x as f32, y as f32, width, height);
    match intersects(ray, &objects)  {
      None => black.to_color(), // empty background
      Some(inter) => illum_out(&inter, &objects, &lights).to_color(),
    }
  }).save("out.png").unwrap();
}
