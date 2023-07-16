 rust
fn at_and(x: &mut int, y: @mut int) {
    *x += 1;
    printfln!(*y);
}

fn at_at(x: @mut int, y: @mut int) {
     *x += 1;
    printfln!(*y);
}

fn main() {
    let x = @mut 1;
    at_at(x, x); // prints 2
    at_and(x, x); // the *y fails
}
