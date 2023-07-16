 rust
struct ExternalNode {
    key: int,
    value: int
}

fn main() {
    let mut boxed_node = box ExternalNode{ key : 1, value: 2};
    let node = &mut *boxed_node;

    match *node {
        ExternalNode{key : ref fkey, value: ref mut val} => {
            *val+=1;
        }
    }
}
