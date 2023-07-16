 rust
struct Node {
    children: ~[Rc<Node>],
    parent: Rc<Node>
}
