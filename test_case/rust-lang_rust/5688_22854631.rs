 rust
struct X { vec: &'static [int] }
static V: &'static [X] = &[X { vec: &[1, 2, 3] }];
fn main() {
    for &v in V.iter() {
        println(fmt!("%?", v.vec));
    }
}
