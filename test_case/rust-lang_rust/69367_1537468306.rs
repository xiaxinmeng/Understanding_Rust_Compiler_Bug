rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    pub next: Option<Rc<RefCell<Node>>>,
}

pub fn foo(root: Option<Rc<RefCell<Node>>>) {
    let mut queue = Vec::new();
    if let Some(x) = root {
        queue.push(x);
    }
    while !queue.is_empty() {
        let x = queue.pop().unwrap();
        if let Some(next) = x.borrow().next.clone() {
            queue.push(next);
        }
        // uncommenting any of the following will compile
        // ()
        // 0;
        // let completely_random_var = 0;
    }
}

fn main() {}
