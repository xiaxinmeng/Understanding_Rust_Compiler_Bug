rust
extern "C" { fn foo(); } 
static FOO: unsafe extern "C" fn() = foo;
pub unsafe fn bar() { FOO() } 
