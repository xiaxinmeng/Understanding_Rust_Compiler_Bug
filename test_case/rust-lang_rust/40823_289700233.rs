
error: cannot borrow immutable borrowed content `*parent` as mutable
   --> src/parser.rs:175:29
    |
174 |                         if let Some (&Node::RegularNode(ref parent)) = nodestack.last() {
    |                                                         ---------- consider changing this to `mut ref parent`
175 |                             parent.add_child (Node::RegularNode(node));
    |                             ^^^^^^ cannot borrow as mutable
