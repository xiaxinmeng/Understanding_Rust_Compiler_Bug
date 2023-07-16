 rust
struct B<F: Fn()> (F);

fn main() {
    let mut p = B(std::mem::zeroed());
    p.0 = ||{p.0;} ;
}
