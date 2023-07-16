
enum Tree<T,U> {
        Leaf(T),
        Branch(~Tree<U,T>),
}
