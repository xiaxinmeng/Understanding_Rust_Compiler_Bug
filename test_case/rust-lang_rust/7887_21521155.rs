
enum Tree<T> {
    Node(T, ~Tree<T>, ~Tree<T>),
    Empty
}

impl<T> Tree<T> {
    fn consume_iter(self) -> ConsumeIterator<T> {
        fn aux(node: Tree<T>, mut values: ~[T]) {
            match node {
                Empty => values,
                Node(value, left, right) => {
                    values = aux(left);
                    values.push(value);
                    values = aux(right);
                    values
                }
            }
        }
        aux(self, ~[]).consume_iter()
    }
}
