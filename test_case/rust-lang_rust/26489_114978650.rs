 Rust
struct B<'a, F: Fn()+'a>(&'a F);

fn main() {
 let mut p = B(std::mem::zeroed());
 p.0 = &||{p.0;};
}
