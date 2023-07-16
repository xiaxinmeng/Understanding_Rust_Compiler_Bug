 rust
#![crate_type = "lib"]

enum NodeContents<'a> {
    Children(Vec<Node<'a>>),
}

impl<'a> Drop for NodeContents<'a> {
  fn drop( &mut self ) {
  }
}

struct Node<'a> {
    contents: NodeContents<'a>,
}

impl<'a> Node<'a> {
    fn noName(contents: NodeContents<'a>) -> Node<'a> {
        Node{  contents: contents,}
    }
}
