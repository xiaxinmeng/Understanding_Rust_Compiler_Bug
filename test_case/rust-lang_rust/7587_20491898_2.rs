
enum Tree<T> {
        Leaf(T),
        Branch(~Tree<Tree<T>>),
}
