rust
error: `cargo` import is ambiguous
  --> tools\rls\src\build\mod.rs:15:5
   |
15 | use cargo::util::important_paths;
   |     ^^^^^ can refer to external crate `::cargo`
...
35 | mod cargo;
   | ---------- may refer to `self::cargo` in the future
   |
   = help: write `::cargo` or `self::cargo` explicitly instead
   = note: in the future, `#![feature(uniform_paths)]` may become the default
error: aborting due to previous error
[RUSTC-TIMING] rls test:false 5.373
error: Could not compile `rls`.
