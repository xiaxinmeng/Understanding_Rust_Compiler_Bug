
use std::sync::Arc;
struct F;
trait Freeze {}
trait Foo: Send+Freeze{}
impl Freeze for F {}
impl Foo for F {}
fn main() {
    let _ = Arc::new(Box::new(F) as Box<Foo>);
}
