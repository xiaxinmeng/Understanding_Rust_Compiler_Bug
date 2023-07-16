 rust
 enum Tree<T> {
     Single(T),
     Deep(Box<Tree<Node<T>>>)
}
