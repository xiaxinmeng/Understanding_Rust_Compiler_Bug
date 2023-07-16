plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
---- src/mem/mod.rs - mem::SizedTypeProperties::IS_ZST (line 1249) stdout ----
error[E0282]: type annotations needed
  --> src/mem/mod.rs:1267:10
   |
21 | assert!(!String::IS_ZST);
   |          ^^^^^^ cannot infer type for type parameter `A` declared on the struct `String`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/mem/mod.rs - mem::SizedTypeProperties::IS_ZST (line 1249)

test result: FAILED. 3950 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 56.17s

error: doctest failed, to rerun pass `-p core --doc`
