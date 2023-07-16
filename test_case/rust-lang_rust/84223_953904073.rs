rust
const INIT: $t = $init;
#[thread_local]
static mut VAL: $t = INIT;
