 bash
../gccrs/gcc/testsuite/rust.test/compile/unused.rs:2:1: warning: function is never used: `bar`
    2 | fn bar() { // { dg-warning "function is never used: `bar`" "" { target *-*-* } .-1 }
      | ^
../gccrs/gcc/testsuite/rust.test/compile/unused.rs:6:1: warning: function is never used: `foo`
    6 | fn foo() { // { dg-warning "function is never used: `foo`" "" { target *-*-* } .-1 }
      | ^
