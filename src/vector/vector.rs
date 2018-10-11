use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}


impl ops::Add<Vector> for Vector {
  type Output = Vector;

  fn add(self, o: Vector) -> Vector {
    Vector{
      x: self.x + o.x,
      y: self.y + o.y,
      z: self.z + o.z,
    }
  }
}

impl<'a, 'b> ops::Add<Vector> for &'a Vector {
  type Output = Vector;

  fn add(self, o: Vector) -> Vector {
    Vector{
      x: self.x + o.x,
      y: self.y + o.y,
      z: self.z + o.z,
    }
  }
}

impl ops::Sub<Vector> for Vector {
  type Output = Vector;

  fn sub(self, o: Vector) -> Vector {
    Vector{
      x: self.x - o.x,
      y: self.y - o.y,
      z: self.z - o.z,
    }
  }
}

impl<'a> ops::Sub<&'a Vector> for Vector {
  type Output = Vector;

  fn sub(self, o: &Vector) -> Vector {
    Vector{
      x: self.x - o.x,
      y: self.y - o.y,
      z: self.z - o.z,
    }
  }
}

impl<'a, 'b> ops::Sub<&'a Vector> for &'b Vector {
  type Output = Vector;

  fn sub(self, o: &Vector) -> Vector {
    Vector{
      x: self.x - o.x,
      y: self.y - o.y,
      z: self.z - o.z,
    }
  }
}

impl ops::Mul<Vector> for Vector {
  type Output = Vector;

  fn mul(self, o: Vector) -> Vector {
    Vector{
      x: self.x * o.x,
      y: self.y * o.y,
      z: self.z * o.z,
    }
  }
}

impl ops::Mul<f32> for Vector {
  type Output = Vector;

  fn mul(self, o: f32) -> Vector {
    Vector {
      x: self.x * o,
      y: self.y * o,
      z: self.z * o,
    }
  }
}

impl<'a> ops::Mul<f32> for &'a Vector {
  type Output = Vector;

  fn mul(self, o: f32) -> Vector {
    Vector {
      x: self.x * o,
      y: self.y * o,
      z: self.z * o,
    }
  }
}

impl ops::Div<f32> for Vector {
  type Output = Vector;

  fn div(self, o: f32) -> Vector {
    Vector {
      x: self.x / o,
      y: self.y / o,
      z: self.z / o,
    }
  }
}

impl Vector {
  pub fn new(x: f32, y: f32, z: f32) -> Vector {
    Vector{x, y, z}
  }
  pub fn dot(&self, o: &Vector) -> f32 {
    self.x * o.x + self.y * o.y + self.z * o.z
  }
  pub fn cross(&self, o: &Vector) -> Vector {
    Vector{
      x: self.y * o.z - self.z * o.y,
      y: self.z * o.x - self.x * o.z,
      z: self.x * o.y - self.y * o.x,
    }
  }
  pub fn sqr_magn(&self) -> f32 {
    return self.dot(self)
  }
  pub fn normalized(self) -> Vector {
    self/self.sqr_magn().sqrt()
  }
  pub fn inv(self) -> Vector {
    self * (-1.0)
  }
  pub fn reflect_across(&self, across: &Vector) -> Vector {
    let norm = across.normalized();
    self - &(norm * (2.0 * self.dot(&norm)))
  }
  pub fn lerp(&self, o: &Vector, percentage_o: f32) -> Vector {
    let percentage_self = 1.0 - percentage_o;
    return self * percentage_self + o * percentage_o;
  }
  pub fn pow(&self, k: f32) -> Vector {
    Vector{
      x: self.x.powf(k),
      y: self.y.powf(k),
      z: self.z.powf(k),
    }
  }
}

pub fn origin() -> Vector {
  Vector{x: 0.0, y: 0.0, z:0.0}
}

enum Dimension { X, Y, Z, }

fn partial_derivative(f: &Fn(&Vector) -> f32, at: &Vector, dim: Dimension) -> f32 {
  let epsilon = 0.001;
  let differ = match dim {
    Dimension::X => Vector::new(epsilon, 0.0, 0.0),
    Dimension::Y => Vector::new(0.0, epsilon, 0.0),
    Dimension::Z => Vector::new(0.0, 0.0, epsilon),
  };
  let a = f(&(at + differ));
  let b = f(&(at - &differ));
  (a - b)/(2.0 * epsilon)
}

// Calculates gradient of F at specified point
pub fn gradient(f: &Fn(&Vector) -> f32 , at: &Vector) -> Vector {
  Vector{
    x: partial_derivative(f, at, Dimension::X),
    y: partial_derivative(f, at, Dimension::Y),
    z: partial_derivative(f, at, Dimension::Z),
  }.normalized()
}

#[test]
fn test_gradient() {
  let sample = |v: &Vector| -> f32 { v.x * v.y * v.z };
  println!("{:?}", gradient(&sample, &Vector::new(2.0, 3.0, 5.0)));
}

