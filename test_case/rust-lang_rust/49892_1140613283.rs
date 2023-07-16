rust
#[derive(PartialEq, Clone, Copy)]
pub enum SbTag {
    Tagged(NonZeroU64),
    Untagged,
}
