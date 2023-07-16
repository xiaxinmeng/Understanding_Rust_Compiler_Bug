rust
#![feature(box_syntax)]

fn main() {
    let _: Box<String> = box String::new();
}
