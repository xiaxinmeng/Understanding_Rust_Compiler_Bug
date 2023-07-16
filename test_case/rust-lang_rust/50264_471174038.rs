rust
let parent_node: &Inert<Option<Dom<Node>>;

// Without Option::deref:
(**parent_node).as_ref().map(|node| &**node);

// With Option::deref:
parent_node.deref();
