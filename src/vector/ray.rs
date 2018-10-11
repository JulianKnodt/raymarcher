use vector::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
  pub position: Vector,
  pub direction: Vector,
}

const SHIFT_EPSILON: f32 = 0.00001;
impl Ray {
  pub fn at(&self, t: f32) -> Vector {
    return self.position + self.direction * t;
  }
  pub fn from(pos: Vector, to: Vector) -> Ray {
    Ray{
      position: pos,
      direction: (to - pos).normalized(),
    }
  }
  pub fn eps_shift(self) -> Ray {
    Ray{
      position: self.at(SHIFT_EPSILON),
      direction: self.direction,
    }
  }
}
