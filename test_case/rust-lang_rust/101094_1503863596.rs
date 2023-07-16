rust
use std::any::Any;
fn foo(x: &mut dyn std::any::Any) {
    println!("{:?}", x.type_id());
}
