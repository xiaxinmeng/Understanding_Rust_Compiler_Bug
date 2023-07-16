rs
#![feature(if_let_guard)]

struct NotCopy;

fn main() {
    match Some(NotCopy) {
        Some(x) if let () = drop(x) => {}
        _ => {}
    }
}
