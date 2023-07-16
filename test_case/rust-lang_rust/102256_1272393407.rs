rust
fn foo(mut n: Option<usize>) {
    let _ = if let Some(ref mut s) = n {
        s
    } else {
        &mut 0
        //~^ ERROR temporary value dropped while borrowed
    };
}
