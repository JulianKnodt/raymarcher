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

  const NUM_RAYS: i32 = 10;
  ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
    let sum = cam.get_rays_to(x as f32, y as f32, width, height, NUM_RAYS)
      .iter()
      .map(|ray| {
        match intersects(*ray, &objects)  {
          None => black, // empty background
          Some(inter) => illum_out(&inter, &objects, &lights, 0),
        }
      })
      .fold(Vector::new(0.0, 0.0, 0.0), |acc, next| acc + next);

      (sum * (1.0 / (NUM_RAYS as f32))).to_color()
  }).save("out.png").unwrap();
}
