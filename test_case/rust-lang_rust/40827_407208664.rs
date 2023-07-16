rust
use std::rc::Rc;
use std::sync::Arc;

struct List {
    value: Rc<i32>,
    next:  Option<Arc<List>>
}

fn is_send() {
    let _: Box<Send> = Box::new(List { value: Rc::new(4), next: None });
}
