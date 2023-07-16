rust
use std::cell::RefCell;

macro_rules! drop_temps {
    ($e:expr) => {{
        let x = $e;
        x
    }}
}

fn main() {
    let x = RefCell::new(true);
    
    match drop_temps!(*x.borrow()) {
        _ => x.borrow_mut()
    };
}
