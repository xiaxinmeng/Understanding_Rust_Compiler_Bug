 rust
pub struct Point {
  x: f64,
  y: f64,
  z: f64
}

impl Mul<Point, Point> for Point {
  fn mul(&self, other: &Point) -> Point {
    Point {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
  }
}

impl Mul<f64, Point> for Point {
  fn mul(&self, v: &f64) -> Point {
    Point {x: self.x * *v, y: self.y * *v, z: self.z * *v}
  }
}

fn main() {

}

// testcase.rs:13:1: 17:2 error: conflicting implementations for trait `std::ops::Mul`
// testcase.rs:13 impl Mul<f64, Point> for Point {
// testcase.rs:14   fn mul(&self, v: &f64) -> Point {
// testcase.rs:15     Point {x: self.x * *v, y: self.y * *v, z: self.z * *v}
// testcase.rs:16   }
// testcase.rs:17 }
// testcase.rs:7:1: 11:2 note: note conflicting implementation here
// testcase.rs:7 impl Mul<Point, Point> for Point {
// testcase.rs:8   fn mul(&self, other: &Point) -> Point {
// testcase.rs:9     Point {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
// testcase.rs:10   }
// testcase.rs:11 }
// testcase.rs:7:1: 11:2 error: conflicting implementations for trait `std::ops::Mul`
// testcase.rs:7 impl Mul<Point, Point> for Point {
// testcase.rs:8   fn mul(&self, other: &Point) -> Point {
// testcase.rs:9     Point {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
// testcase.rs:10   }
// testcase.rs:11 }
// testcase.rs:13:1: 17:2 note: note conflicting implementation here
// testcase.rs:13 impl Mul<f64, Point> for Point {
// testcase.rs:14   fn mul(&self, v: &f64) -> Point {
// testcase.rs:15     Point {x: self.x * *v, y: self.y * *v, z: self.z * *v}
// testcase.rs:16   }
// testcase.rs:17 }
// testcase.rs:7:1: 11:2 error: expected std::ops::Mul<Point,Point>, but found std::ops::Mul<f64,Point> (expected struct Point but found f64)
// testcase.rs:7 impl Mul<Point, Point> for Point {
// testcase.rs:8   fn mul(&self, other: &Point) -> Point {
// testcase.rs:9     Point {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
// testcase.rs:10   }
// testcase.rs:11 }
// testcase.rs:7:1: 11:2 error: expected std::ops::Mul<Point,Point>, but found std::ops::Mul<f64,Point> (expected struct Point but found f64)
// testcase.rs:7 impl Mul<Point, Point> for Point {
// testcase.rs:8   fn mul(&self, other: &Point) -> Point {
// testcase.rs:9     Point {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
// testcase.rs:10   }
// testcase.rs:11 }
// testcase.rs:7:1: 11:2 error: multiple applicable methods in scope
// testcase.rs:7 impl Mul<Point, Point> for Point {
// testcase.rs:8   fn mul(&self, other: &Point) -> Point {
// testcase.rs:9     Point {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
// testcase.rs:10   }
// testcase.rs:11 }
// testcase.rs:13:1: 17:2 error: expected std::ops::Mul<f64,Point>, but found std::ops::Mul<Point,Point> (expected f64 but found struct Point)
// testcase.rs:13 impl Mul<f64, Point> for Point {
// testcase.rs:14   fn mul(&self, v: &f64) -> Point {
// testcase.rs:15     Point {x: self.x * *v, y: self.y * *v, z: self.z * *v}
// testcase.rs:16   }
// testcase.rs:17 }
// testcase.rs:13:1: 17:2 error: expected std::ops::Mul<f64,Point>, but found std::ops::Mul<Point,Point> (expected f64 but found struct Point)
// testcase.rs:13 impl Mul<f64, Point> for Point {
// testcase.rs:14   fn mul(&self, v: &f64) -> Point {
// testcase.rs:15     Point {x: self.x * *v, y: self.y * *v, z: self.z * *v}
// testcase.rs:16   }
// testcase.rs:17 }
// testcase.rs:13:1: 17:2 error: multiple applicable methods in scope
// testcase.rs:13 impl Mul<f64, Point> for Point {
// testcase.rs:14   fn mul(&self, v: &f64) -> Point {
// testcase.rs:15     Point {x: self.x * *v, y: self.y * *v, z: self.z * *v}
// testcase.rs:16   }
// testcase.rs:17 }
// error: aborting due to 8 previous errors
// task 'rustc' failed at 'explicit failure', /build/rust/src/rust-0.9/src/libsyntax/diagnostic.rs:75
// task '<main>' failed at 'explicit failure', /build/rust/src/rust-0.9/src/librustc/lib.rs:453
