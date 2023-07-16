rust
trait T<'a> { type A; }
impl T<'_> for u8 {  type A = u32; }

fn main() {
    let _x: fn(<u8 as T<'_>>::A) = |_| {};
}
