rust
let x: i32 = std::mem::MaybeUninit::frozen().assume_init();
