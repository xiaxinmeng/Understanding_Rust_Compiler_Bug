rust
let update_something = Rc::new(move || {
    // ...
});
let clone_update_something = || {
    let update_something = update_something.clone();
    move |_| update_something()
};
toggle1.connect_toggled(clone_update_something());
toggle2.connect_toggled(clone_update_something());
toggle3.connect_toggled(clone_update_something());
