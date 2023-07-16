rust
struct A<F>(F);

unsafe impl <'a, 'b, F: Fn(&'a i32) -> &'b i32> Send for A<F> {}

fn wrapped_closure() -> impl Sized {
    let z = 0;
    let f = |x| x;
    f(&z);
    A(f)
}

fn main() {
    let x: Box<dyn Send> = Box::new(wrapped_closure());
}
