rust
// Works
fn f1() -> bool {
    macro_rules! try(( $ e : expr ) => (
        match $ e {
            Some ( e ) => e ,
            None => return false ,
        }
    ));

    try!(Some(42)) == 42
}

// Expanded version doesn't work
fn f2() -> bool {
    match Some(42) {
        Some ( e ) => e ,
        None => return false ,
    } == 42
}

// Could be OK with parenthesis (at least in this case)
fn f3() -> bool {
    (match Some(42) {
        Some ( e ) => e ,
        None => return false ,
    }) == 42
}
