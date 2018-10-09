use std::f32;

type RealFn<'a> = &'a Fn(f32) -> f32;

const MAX_ITERATION: i32 = 5000;
// finds the zero of realFn
pub fn bisection(f: RealFn, x0: f32, x1: f32, threshold: f32) -> f32 {
  let (mut positive, mut negative) = match (f(x0), f(x1)) {
    (p, n) if p >= 0.0 && n <= 0.0 => ((x0, p), (x1, n)),
    (n, p) if n <= 0.0 && p >= 0.0 => ((x1, p), (x0, n)),
    (a, b) if a.signum() == b.signum() => panic!("x0 and x1's images have the same sign"),
    (_, _) => panic!("Should never be reached"),
  };
  let mut iter = 0;
  let mut mid = (positive.0 + negative.0)/2.0;
  let mut mid_val = f(mid);
  while iter < MAX_ITERATION && mid_val.abs() > threshold {
    iter += 1;
    match mid_val.signum() {
      x if x.is_nan() => panic!("NaN in bisection"),
      x if x > 0.0 => {
        positive.0 = mid;
        positive.1 = mid_val;
      },
      x if x < 0.0 => {
        negative.0 = mid;
        negative.1 = mid_val;
      },
      _ => panic!("TODO when x is 0"),
    };
    mid = (positive.0 + negative.0)/2.0;
    mid_val = f(mid);
  }
  mid
}

const EPSILON: f32 = 0.0000000001;
pub fn newton(f: RealFn, x0: f32, threshold: f32) -> f32 {
  let mut curr = x0;
  let mut curr_val = f(curr);
  let mut iter = 0;
  while curr_val.abs() > threshold && iter < MAX_ITERATION {
    iter = iter + 1;
    curr = curr - f(curr)/deriv_approx(f, curr, EPSILON);
    let next_val = f(curr);
    if next_val.abs() > curr_val.abs() {
      // Diverging.
      break;
    }
    curr_val = next_val;
  }
  curr
}

pub fn deriv_approx(f: RealFn, x: f32, epsilon: f32) -> f32 {
  (f(x+epsilon) - f(x-epsilon))/(2.0 * epsilon)
}

#[test]
fn test_bisection() {
  let test_fn = |x: f32| -> f32 {
    x.powf(2.0) - 10.0
  };
  println!("{}", bisection(&test_fn, 3.0, 3.5, 0.01));
}

// pub fn newton(opt: &Fn(f32) -> f32) -> f32 {
//
// }

