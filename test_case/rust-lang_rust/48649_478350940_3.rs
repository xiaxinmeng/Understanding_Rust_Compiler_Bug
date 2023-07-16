rust
pub struct ZipSubSlice {
  left: [u8; 8],
  right: [u8; 8],
  pub range: Range<usize>,
}

impl Copy for ZipSubSlice {} // currently errors
