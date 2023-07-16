rust
mod bad { extern "C" { fn foo(); } } // never unwinds
extern "C+unwind" { fn foo(); }  // BOOM: becomes nounwind because of bad::foo
