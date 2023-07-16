plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* 629 error codes
* highest error code: E0787
Found 503 error codes
Found 0 error codes with no tests
tidy error: /checkout/compiler/rustc_typeck/src/collect.rs: too many lines (3003) (add `// ignore-tidy-filelength` to the file to suppress this error)
Done!
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: Zero }', src/tools/tidy/src/features.rs:387:32
Build completed unsuccessfully in 0:00:11
