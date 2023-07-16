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
................................................................................iiiiii.. 1144/1221
.............................................i...............................
failures:

---- src/macros.rs - macros::eprint (line 166) stdout ----
error[E0599]: no method named `flush` found for struct `Stderr` in the current scope
     |
     |
8    | io::stderr().flush().unwrap();
     |              ^^^^^ method not found in `Stderr`
    ::: /checkout/library/std/src/io/mod.rs:1506:8
     |
1506 |     fn flush(&mut self) -> Result<()>;
     |        ----- the method is available for `Stderr` here
---
    src/macros.rs - macros::eprint (line 166)

test result: FAILED. 1202 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out; finished in 13.44s

error: doctest failed, to rerun pass `-p std --doc`
