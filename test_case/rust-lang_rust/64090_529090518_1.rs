rust
extern "C" { fn foo(); } 
pub unsafe fn bar() { foo() } // bar is nounwind because foo is nounwind
