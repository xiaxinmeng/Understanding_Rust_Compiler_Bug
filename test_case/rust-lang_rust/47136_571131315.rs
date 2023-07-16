rust
trait Trait {}

struct Struct;
impl Trait for Struct {}

fn main() {
    let s = Struct; // or `let mut s = Struct;`

    let _ = s as &dyn Trait; // or `let _ = s as &mut dyn Trait;`
}
