rust
fn memory_size<T: Memory>() -> i32;
fn memory_grow<T: Memory>(delta: i32) -> i32;

trait Memory: Sealed { /* all unstable */ }
struct Memory0;

impl Memory for Memory0 { /* ... */ }

memory_size::<Memory0>(); 
// etc...
