rust
struct Tree(u16, [Option<Box<Tree>>]);

struct Node<T: ?Sized> {
    population: u16,
    children: T,
}

struct Tree2(Node<Tree>);

fn main() {
    use std::mem::size_of;
    use std::sync::atomic;
    println!("{:?}", size_of::<&Tree>());
    println!("{:?}", size_of::<&Tree2>());
}
