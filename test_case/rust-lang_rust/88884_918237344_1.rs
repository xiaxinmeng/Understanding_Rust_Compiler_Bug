rust
// will not compile
fn ident<'a>(arg: &'a &'a ()) -> &'a &'a () { arg }

// will compile
// two different lifetime parameters
fn ident<'a, 'b>(arg: &'a &'b ()) -> &'a &'b () { arg }

// will not compile
// Return a reference with the shorter of the two lifetime parameters (if my understanding of lifetimes is correct)
fn ident<'a, 'b: 'a>(arg: &'a &'b ()) -> &'a &'a () { arg }

fn do_nothing<'z>(arg: &'z ()) {
    (|s: &'z ()| s)(*ident(&arg));
}
