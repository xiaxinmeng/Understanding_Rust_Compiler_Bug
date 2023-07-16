 rust
struct Node {
    children: ~[Strong<Node>],
    parent: Weak<Node>
}
