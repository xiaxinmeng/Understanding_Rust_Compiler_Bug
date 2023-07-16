 rust

use std::collections::HashMap;
use std::rc::Rc;
fn foo<T: Send>() {}
fn main() {
    foo::<HashMap<Rc<()>, Rc<()>>>();
}
