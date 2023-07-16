
#![feature(box_syntax)]
#![feature(box_patterns)]
struct ExternalNode {
    key: i32,
    value: i32
}

fn main() {
    let mut boxed_node = box ExternalNode{ key : 1, value: 2};

    match boxed_node {
        box ExternalNode{key : ref fkey, value: ref mut val} => {
            *val+=1;
        }
    }

}
