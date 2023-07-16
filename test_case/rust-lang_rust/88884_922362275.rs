rust
fn ident<'a>(arg: &'a &'a ()) -> &'a &'a () { arg }

fn do_nothing<'z: 'y, 'y>(arg: &'z ()) {
    (|s: &'y ()| s)(ident(&arg));
}
