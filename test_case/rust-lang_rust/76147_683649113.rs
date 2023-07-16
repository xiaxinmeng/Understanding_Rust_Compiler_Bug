rust
fn ok<'a>(v: &'a mut ()) -> &'a mut () {
    v
}

pub fn union<'a>(v: &'a mut ()) {
    let x: *mut &'a mut () = &mut ok(v);
    let _ = (x, v);
}
