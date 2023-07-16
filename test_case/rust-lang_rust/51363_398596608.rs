 rust
extern "C" fn keep_this() {}

#[used]
static KEEP: extern "C" fn() = keep_this;
