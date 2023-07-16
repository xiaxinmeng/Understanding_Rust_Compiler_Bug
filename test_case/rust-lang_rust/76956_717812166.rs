rust
use std::ops::Deref;

fn use_data(v: &'static i32, user: &dyn for<'a> Fn(<Box<&'a i32> as Deref>::Target)) {
    user(v)
}
