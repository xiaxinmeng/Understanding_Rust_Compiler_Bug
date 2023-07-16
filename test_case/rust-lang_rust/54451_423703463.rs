 rust
fn foo() {} // WARN function is never used

#[no_mangle]
static FOO: fn() = foo; // WARN static item is never used

struct Bar; // WARN struct is never constructed

#[no_mangle]
static BAR: Bar = Bar; // WARN static item is never used

#[no_mangle]
fn baz() {} // WARN function is never used
