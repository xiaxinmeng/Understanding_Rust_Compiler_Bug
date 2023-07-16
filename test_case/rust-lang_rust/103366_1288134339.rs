rust
fn<'a, B: 'a + std::ops::Add<Output = u32>> f(_x: B) { }

fn main() {
    f(1_u32);
}
