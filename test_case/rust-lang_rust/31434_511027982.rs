rust
use std::rc::Rc;
use std::error::Error;

const OK: Result<(), Rc<dyn Error>> = Ok(());

fn main() {
    let is_ok = match OK {
        OK => true,
        _ => false,
    };
}
