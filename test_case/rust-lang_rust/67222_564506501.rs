rust
fn main() {
    let s = std::cell::RefCell::new(Some(()));
    
    if let Some(_) = s.borrow().as_ref() {} else {} // add `;` to make it compilable
}
