rust
let rc = Rc::new(());
let &None = &Some(Rc::clone(&rc)) else {
    Rc::try_unwrap(rc).unwrap();
    return;
};
unreachable!();
