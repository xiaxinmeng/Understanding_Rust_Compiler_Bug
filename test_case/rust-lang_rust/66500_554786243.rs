rust
#![feature(untagged_unions)]

union U<'a> {
    x: &'a mut String,
}

fn f(u: U<'_>) -> String {
    unsafe { *u.x }
}
