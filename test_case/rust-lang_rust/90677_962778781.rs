rust
fn f() -> Option<(i32, i8)> {
    Some(1, 2)
    //~^ ERROR this enum variant takes 1 argument but 2 arguments were supplied
}
