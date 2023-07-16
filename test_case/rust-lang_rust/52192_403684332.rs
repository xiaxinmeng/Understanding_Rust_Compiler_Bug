rust
fn foo() -> i32 {
    let x;
    while true {
        x = 4;
        break;
    }
    x // error: use of possibly uninitialized variable
}
