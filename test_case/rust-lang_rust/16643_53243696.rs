 rust
// euv_ice.rs
#![crate_type="lib"]

pub struct TreeBuilder<H>;

impl<H> Iterator<H> for TreeBuilder<H> {
    fn next(&mut self) -> Option<H> { None }
}

impl<H> TreeBuilder<H> {
    pub fn process_token(&mut self) {
        match self {
            _ => for _y in *self {},
        }
    }
}
