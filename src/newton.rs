type RealFn<'a> = &'a Fn(f32) -> f32;

const max_iteration: i32 = 5000;
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
  while iter < max_iteration && mid_val.abs() > threshold {
    iter += 1;
    match mid.signum() {
      1.0 => {
        positive.0 = mid;
        positive.1 = mid_val;
      },
      -1.0 => {
        negative.0 = mid;
        negative.1 = mid_val;
      },
      std::f32::NAN => panic!("NaN in bisection"),
    };
    mid = (positive.0 + negative.0)/2.0;
    mid_val = f(mid);
  }
  mid
}

#[test]
fn test_bisection() {
  let test_fn = |x: f32| -> f32 {
    x.pow(2.0) - 10
  };
  println!(bisection(&test_fn, 3.0, 3.5, 0.01));
}

// pub fn newton(opt: &Fn(f32) -> f32) -> f32 {
//
// }
