 rust
#[inline(never)]
#[no_mangle]
fn fmtarg0() -> ~str {
    fmt!("Hello world for the 4th time again!")
}

#[inline(never)]
#[no_mangle]
fn ifmtarg0() -> ~str {
    ifmt!("Hello world for the 4th time again!")
}

#[inline(never)]
#[no_mangle]
fn fmtarg1() -> ~str {
    fmt!("Hello %s for the 4th time again!", "world")
}

#[inline(never)]
#[no_mangle]
fn ifmtarg1() -> ~str {
    ifmt!("Hello {:s} for the 4th time again!", "world")
}

#[inline(never)]
#[no_mangle]
fn fmtarg2() -> ~str {
    fmt!("Hello %s for the %dth time again!", "world", 4)
}

#[inline(never)]
#[no_mangle]
fn ifmtarg2() -> ~str {
    ifmt!("Hello {:s} for the {:d}th time again!", "world", 4)
}

#[inline(never)]
#[no_mangle]
fn fmtarg3() -> ~str {
    fmt!("Hello %s for the %dth time %s!", "world", 4, "again")
}

#[inline(never)]
#[no_mangle]
fn ifmtarg3() -> ~str {
    ifmt!("Hello {:s} for the {:d}th time {:s}!", "world", 4, "again")
}

fn main() {
}
