 rust
enum Opt<T> {
    Empty,
    Val { val: T }
}

struct UniqueNode<T> {
    next: Opt<Box<UniqueNode<T>>>,
    value: T
}


fn main() {
    let stack_unique: UniqueNode<u16> = UniqueNode {
        next: Val {
            val: box UniqueNode {
                next: Empty,
                value: 1,
            }
        },
        value: 0,
    };
}
