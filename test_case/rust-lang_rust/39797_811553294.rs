rust
use turbonone::turbonone;

trait T {
  fn m<S>(&self, s: S);
}

fn f<X: T>(t: Option<Box<X>>) {
    println!("well");
}

fn main() {
    f(turbonone!(Box));
}
