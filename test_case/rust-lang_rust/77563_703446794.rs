rust
struct ZST;
impl ZST {
  pub const fn new() -> Self { Self }

  pub const fn static_ref() -> &'static Self {
    const VAL: ZST = ZST::new();
    &VAL
  }
}
