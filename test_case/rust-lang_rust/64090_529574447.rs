rust
extern "C" { fn foo(); } // Declared to never unwind
static BAR: extern "C+unwind" unsafe fn() = foo as _; // this cast is sound
unsafe fn bar() { BAR() } // BOOM: bar is optimized as nounwind
