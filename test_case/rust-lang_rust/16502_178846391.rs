
#![feature(box_syntax)]

fn main() {
    let p = box 1;

    let _ = p; // doesn't move
    p; // does move


    p; // error: use of moved value
}
