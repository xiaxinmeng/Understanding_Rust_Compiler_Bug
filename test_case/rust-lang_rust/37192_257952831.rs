 rust
let my_rc: *const T = ...;
let my_rc_reified = Rc::from_raw(my_rc);
let my_clone = my_rc_reified.clone();
let my_rc_again = Rc::into_raw(my_rc_reified);
