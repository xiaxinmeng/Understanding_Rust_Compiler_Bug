 rust
let foo = Mut::new(Some(10));
do spawn {
    let foo = foo.borrow_mut();
    let foo = foo.get().take();
    ...
}
