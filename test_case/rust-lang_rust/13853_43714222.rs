
#![crate_type = "lib"]

enum NodeContents<'a> {
    Children(Vec<Node<'a>>),
}

struct Node<'a> {
    contents: NodeContents<'a>,
}

impl<'a> Node<'a> {
    fn noName(contents: NodeContents<'a>) -> Node<'a> {
        Node{  contents: contents,}
    }
}
