rust
impl Testable for SimpleTest {
   fn run(&self) {
      self.test_fn()
   }

  fn name(&self) -> String {
     self.name.to_string()
  }

  fn is_bench(&self) -> bool {
     self.is_bench
  }
}

impl Testable for CriterionTest {
  fn run(&self) {
    self.bench_fn(criterion::global_instance())
  }

  fn name(&self) -> String {
    self.name.to_string()
  }

  fn is_bench() -> bool {
    true
  }
