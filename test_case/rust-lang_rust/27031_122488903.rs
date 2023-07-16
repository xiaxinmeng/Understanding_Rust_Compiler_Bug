 rust
enum Tree<T> {
   Bin(*mut Tree<T>, T, *mut Tree<T>),
   Leaf
}

enum TreeZipper<'a, T : 'a> {
    Top,
    Left(&'a mut TreeZipper<'a, T>, T, Tree<T>),
    Right(Tree<T>, T, &'a mut TreeZipper<'a, T>)
}

fn cast<'a : 'b, 'b, T : 'a>(foo: TreeZipper<'a, T>) -> TreeZipper<'b, T> {
    foo
}
