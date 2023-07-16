rust
let b = unsafe { std::mem::transmute::<i8, Enum>(std::mem::transmute::<Enum, i8>(Enum::VariantB)) };
assert_eq!(1, Enum::VariantB as i8);
assert_eq!(1, b as i8);
