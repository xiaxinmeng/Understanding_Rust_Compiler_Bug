
trait times<IN, OUT> {
  fn *(&&x: IN) -> OUT;
}

impl of times<float, vec3> for vec3 {
  fn *(f: float) -> vec3 {
        { x: self.x * f, y: self.y * f,  z: self.z * f }
  }
}

impl of times<vec3, float> for vec3 {
  fn *(v: vec3) -> float {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
  }
}
