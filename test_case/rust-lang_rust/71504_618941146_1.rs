rust
use globset::GlobSetBuilder;

fn main() {
    assert!(GlobSetBuilder::new().build().unwrap().is_match("src/tests"));
}

