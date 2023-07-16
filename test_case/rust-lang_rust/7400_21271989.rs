 rust
pub trait NodeBase<'self> {
    pub fn base_node(&'self self) -> &'self int;
}

pub fn create<'self, T: NodeBase<'self>>(node: &'self mut T) {
}

fn main() {
}
