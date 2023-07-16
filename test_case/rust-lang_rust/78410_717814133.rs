rust
use std::ops::Deref;

fn use_data(v: &'static i32, user: &dyn for<'a> Fn(<Box<&'a i32> as Deref>::Target)) {
    //~^ ERROR expected function, found `&dyn for<'a> Fn(<Box<&'a i32> as Deref>::Target)`
    user(v)
}

fn use_data(v: &'static i32, user: &dyn for<'a> Fn(&'a i32)) {
    // OK
    user(v)
}
