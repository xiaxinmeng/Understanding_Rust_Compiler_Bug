
enum Tree<T> {
        Leaf(T),
        Branch(~Tree<~T>),
}
