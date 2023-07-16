 rust
trait Trait {}

struct Bar;
impl Trait for Bar {}

const FOO: &'static Trait = &Bar;
const BAR: &'static Trait = FOO;
fn main() { let x = BAR; }
