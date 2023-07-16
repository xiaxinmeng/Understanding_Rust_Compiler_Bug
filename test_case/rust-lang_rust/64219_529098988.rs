rust
extern "C" { static FOO: fn() -> !; }
pub unsafe fn foo() { FOO(); }
