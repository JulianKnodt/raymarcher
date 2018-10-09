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
}

pub fn origin() -> Vector {
  Vector{x: 0.0, y: 0.0, z:0.0}
}
