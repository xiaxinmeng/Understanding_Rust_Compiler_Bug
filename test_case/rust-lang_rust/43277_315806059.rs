rust
use std::heap::System;

#[global_allocator]
static ALLOCATOR: System = System;
