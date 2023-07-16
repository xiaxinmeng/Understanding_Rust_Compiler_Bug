rust
enum Never {}
let x: Never = unsafe { core::mem::uninitialized() };
