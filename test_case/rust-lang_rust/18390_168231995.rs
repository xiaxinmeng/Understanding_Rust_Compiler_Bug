 rust
fn main() {
    let _ = foo();
}

fn foo() -> i32 {
    let x = 5;
    match x {
        1 => {
            1
        },
        2 => {
            2
        },
        3 => {
            3
        },
        4 => {
            4
        },
        5 => {
            5
        },
        _ => {
            0
        },
    };
    match x {
        1 => {
            1
        },
        2 => {
            2
        },
        3 => {
            3
        },
        4 => {
            4
        },
        5 => {
            5
        },
        _ => {
            0
        },
    }; // <- here is the culprit!
}
