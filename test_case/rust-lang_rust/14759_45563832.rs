 rust
struct Evil {
    val: RefCell<Option<Weak<Evil>>>
}

impl<T: fmt::Show> fmt::Show for Evil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.val.borrow().fmt(f)
    }
}

fn main() {
    let a = Rc::new(Evil { val: RefCell::new(None) });
    let b = a.clone().downgrade();
    *a.val.borrow_mut() = Some(b);
    println!("{}", a);
}
