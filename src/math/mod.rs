
#[cfg(test)]
mod tests {
    use super::normalize;

  fn assert_close(a: f64, b: f64, tol: f64, message: &str) {
    if (a - b).abs() > tol {
      panic!("assert_close failed: {} {} and {} are not close enough with a tolerance of {}", message, a, b, tol);
    }
  }
  #[test]
  fn test_rad2deg() {
    let test = crate::math::rad2deg(std::f64::consts::FRAC_PI_2);
    let expected = 90.0;
    let tolerance = 0.001;
    assert_close(test, expected, tolerance, "pi / 2 to degrees");
  }

  #[test]
  fn test_deg2rad() {
    let test = crate::math::deg2rad(90.0);
    let expected = std::f64::consts::FRAC_PI_2;
    let tolerance = 0.001;
    assert_close(test, expected, tolerance, "90 degrees to radians");
  }

  #[test]
  fn test_normalize() {
    let tolerance = 0.001;
    assert_close(normalize(450.0, 360.0), 90.0, tolerance, "normalize 450.0 to [0, 360]");
    assert_close(normalize(-80.0, 360.0), 280.0, tolerance, "normalize -80.0 to [0, 360]");
    assert_close(normalize(750.3, 360.0), 30.3, tolerance, "normalize 750.3 to [0, 360]");
    assert_close(normalize(25.0, 24.0), 1.0, tolerance, "normalize 25.0 to [0, 24]");
  }
}

pub fn rad2deg(rad: f64) -> f64 {
  return rad * (180.0 / std::f64::consts::PI);
}

pub fn deg2rad(deg: f64) -> f64 {
  return deg * (std::f64::consts::PI / 180.0);
}

pub fn normalize(value: f64, max: f64) -> f64 {
  let normalized = value % max;
  if normalized < 0.0 {
    return normalized + max;
  } else {
    return normalized;
  }
}


pub mod deg {
  use crate::math::*;
  pub fn normalize_angle(angle: f64) -> f64 {
    return normalize(angle, 360.0);
  }

  pub fn sin(angle: f64) -> f64 {
    return deg2rad(angle).sin();
  }

  pub fn cos(angle: f64) -> f64 {
    return deg2rad(angle).cos();
  }

  pub fn tan(angle: f64) -> f64 {
    return deg2rad(angle).tan();
  }

  pub fn atan2(y: f64, x: f64) -> f64 {
    return rad2deg(y.atan2(x));
  }

  pub fn asin(v: f64) -> f64 {
    return rad2deg(v.asin())
  }

  pub fn acos(v: f64) -> f64 {
    return rad2deg(v.acos());
  }

  pub fn acot(v: f64) -> f64 {
    return rad2deg((1_f64 / v).atan())
  }
}

pub mod time {
  use crate::math::*;
  pub fn normalize_hour(hour: f64) -> f64 {
    return normalize(hour, 24.0);
  }
}

