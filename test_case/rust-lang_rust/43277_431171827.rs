rust
use std::alloc::System;

#[global_allocator]
static ALLOCATOR: System = System;
