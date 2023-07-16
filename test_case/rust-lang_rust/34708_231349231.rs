 Rust
#![feature(slice_patterns, advanced_slice_patterns, box_syntax, box_patterns)]

fn cond() -> bool { false }

fn make() -> Box<[Box<[Box<[String]>]>]> {
    box [box [box ["hello".to_string(), "world".to_string()]]]
}

fn main() {
    let b3 = make();

    match (b3, cond()) {
        (box [box [box [s, ..], ..], ..], true) => {
            println!("{}", s);
        }
        (box [.., box [.., box [.., s]]], _) => {
            println!("{}", s);
        }
        _ => {}
    }
}
