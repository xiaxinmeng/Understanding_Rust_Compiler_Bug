rust
fn main() {
    use std::rc::Rc;
    use std::cell::RefCell;

    let arr = [
        Rc::new(RefCell::new("Hello")),
        Rc::new(RefCell::new("World"))
    ];
    let s = arr
        .iter()
        .map(|s| {
            let c = Rc::clone(s);
            format!("{}", c.borrow())
            // This line works fine
            //return format!("{}", c.borrow());
        })
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", s);
}
