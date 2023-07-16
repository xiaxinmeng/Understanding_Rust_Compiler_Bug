rust
fn ident<'a>(arg: &'a &'a ()) -> &'a &'a () { arg }

fn do_nothing<'z: 'y, 'y>(arg: &'z ()) {
    ident(&arg) as &'y &'y ();
}
