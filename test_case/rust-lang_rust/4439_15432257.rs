
trait Eat {
    fn f(self);
}

impl Eat for ~int {
    fn f(self) {
        fail_unless!((*self) == 42);
    }
}

fn main() {
  (~42).f();
}
