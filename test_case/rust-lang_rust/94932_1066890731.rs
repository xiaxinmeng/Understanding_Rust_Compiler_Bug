rust
fn main() {
    let x = Some(It { a: 1 });
    match x {
        Some(x) if x == It { a: 0 } => {}
        _ => {}
    }
}
#[derive(PartialEq, Eq)]
struct It { a: i32 }
