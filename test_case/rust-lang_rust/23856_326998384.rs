rust

trait MyFunc: FnMut() {}
impl<F:FnMut()> MyFunc for F {}

// OK
fn takes_generic<F: MyFunc>(mut f: F) { f(); }
// Error: missing associated type `Output` value
fn takes_trait_object(f: &mut MyFunc) { f(); }

fn main() {}
