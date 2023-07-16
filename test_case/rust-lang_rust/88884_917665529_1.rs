
fn ident<'a>(arg: &'a &'a ()) -> &'a &'a () { arg }

fn do_nothing<'z: 'z>(arg: &'z ()) {
    (|s: &'z ()| s)(ident(&arg));
}
