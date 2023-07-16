rust
let update_something = Rc::new(move |_| {
    // ...
});
toggle1.connect_toggled(update_something.clone());
toggle2.connect_toggled(update_something.clone());
toggle3.connect_toggled(update_something.clone());
