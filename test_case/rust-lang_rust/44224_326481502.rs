rust
#[derive(JSTraceable)]
pub enum Filter {
    None,
    Native(fn (node: &Node) -> u16),
    JS(Rc<NodeFilter>)
}
