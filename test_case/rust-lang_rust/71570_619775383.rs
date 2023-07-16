rust
let update_something = Rc::new(move || {
    // ...
});
toggle1.connect_toggled({
    let update_something = update_something.clone();
    move |_| update_something()
});
toggle2.connect_toggled({
    let update_something = update_something.clone();
    move |_| update_something()
});
toggle3.connect_toggled({
    let update_something = update_something.clone();
    move |_| update_something()
});
