plain
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error: unused variable: `min_version`
    --> src/tools/compiletest/src/header.rs:1051:17
     |
1051 |             let min_version = extract_llvm_version(rest).unwrap();
     |                 ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_min_version`
     |
     = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `compiletest` due to previous error
Build completed unsuccessfully in 0:09:19
