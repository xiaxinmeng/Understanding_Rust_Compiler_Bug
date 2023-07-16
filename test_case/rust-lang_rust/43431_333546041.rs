rust
#![feature(fn_traits)]

trait CallSingle<A, B> {
    fn call(&self, a: A) -> B where Self: Fn(A) -> B;
}

impl<A, B, F: Fn(A) -> B> CallSingle<A, B> for F {
    fn call(&self, a: A) -> B {
        <Self as Fn(A) -> B>::call(self, (a,))
    }
}

fn main() {}

---

   Compiling playground v0.0.1 (file:///playground)
error[E0229]: associated type bindings are not allowed here
 --> src/main.rs:9:27
  |
9 |         <Self as Fn(A) -> B>::call(self, (a,))
  |                           ^ associated type not allowed here

error: aborting due to previous error
