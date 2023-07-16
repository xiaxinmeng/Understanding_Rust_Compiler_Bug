rust
const SINGLE: UnsafeCell<NonCopyType> = UnsafeCell::new(NonCopyType::const_new());
let arr: [UnsafeCell<NonCopyType>; 64] = [SINGLE; 64];
