rust
#[rustc_clean(except="Hir, HirBody, TypeOfItem", cfg="cfail3")]
#[rustc_clean(cfg="cfail3")]
struct Abc { ... }
