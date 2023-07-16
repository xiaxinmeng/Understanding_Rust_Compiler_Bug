rust
fn main() {
    let _ = foo();
}

fn foo() -> i32 {
    let x = 5;
    match x {
        1 => {
            return 1
        },
        2 => {
            return 2
        },
        3 => {
            return 3
        },
        4 => {
            return 4
        },
        5 => {
            return 5
        },
        _ => { }
    }
}
