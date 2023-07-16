plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unnecessary parentheses around cast expression
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs:1377:13
     |
1377 |     stmxcsr((&mut result) as *mut _ as *mut i8);
     |             ^           ^
     |
     = note: `-D unused-parens` implied by `-D warnings`
help: remove these parentheses
     |
1377 -     stmxcsr((&mut result) as *mut _ as *mut i8);
1377 +     stmxcsr(&mut result as *mut _ as *mut i8);

error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:03:20
