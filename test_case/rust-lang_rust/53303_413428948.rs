rust
pub mod objects {
    mod impls {
        use ConstructionSite;
    }

    pub struct ConstructionSite(Reference);

    struct Reference;
}

pub use objects::*;

fn main() {}
