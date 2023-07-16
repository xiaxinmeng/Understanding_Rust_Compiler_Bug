plain
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error[E0425]: cannot find value `rustc` in this scope
    --> src/tools/compiletest/src/runtest.rs:2332:17
     |
2332 |                 rustc.arg("-Clink-args=--emit=asm");

For more information about this error, try `rustc --explain E0425`.
error: could not compile `compiletest` due to previous error
Build completed unsuccessfully in 0:05:00
