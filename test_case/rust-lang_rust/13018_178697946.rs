
use std::rc::Rc;

#[inline(never)]
fn foo<T>(t: &Rc<T>) {
    drop(t.clone());  
}

fn main() {
    let r = Rc::new(5);

    foo(&r);
}
