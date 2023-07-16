 rust
#![feature(unboxed_closures)]
struct GradFn<F: Fn() -> uint>(F);

fn main() {
    let g = GradFn(|&:| 2u);
    let GradFn(f) = g; // LLVM error
    println!("{}", f.call(()));
}
