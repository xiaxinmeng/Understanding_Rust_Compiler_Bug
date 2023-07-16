rust
#![warn(rust_2021_incompatible_closure_captures)]

fn main() {}

pub(crate) struct Numberer {}

impl Numberer {
    pub(crate) async fn new(
        interval: Duration,
    ) -> Numberer {
        Numberer {}
    }
}
