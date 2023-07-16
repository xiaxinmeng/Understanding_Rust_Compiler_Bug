rust
struct B<F: Fn(B<F>)>(F);

fn main() {
    let _b = B(|_b| ());
}
