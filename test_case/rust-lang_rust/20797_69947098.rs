 rust
trait DeclaredTrait {
    type Type;
}
impl DeclaredTrait for i32 {
    type Type = i32;
}
struct Struct<B: DeclaredTrait> {
    b: B,
    b1: B::Type,
}
fn main() {
    let e = Struct {
        b: 0,
        b1: 0,
    };
}
