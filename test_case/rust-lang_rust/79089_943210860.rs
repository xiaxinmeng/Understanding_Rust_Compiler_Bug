rust
fn require_types_equal<T>(x: T, y: T) {}
 
fn main() {
    let x: c_char = 0;
    let y = 0_u8 as _;
    require_types_equal(x, y);
}
