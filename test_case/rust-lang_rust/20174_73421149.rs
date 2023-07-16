 rust
struct GradFn<F: Fn() -> usize>(F);

fn main() {
    let grad_fn = || -> usize { 2us };
    let GradFn(x_squared) : GradFn<_> = GradFn(grad_fn);
    let _  = x_squared();
}
