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
    Finished release [optimized] target(s) in 8.80s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: duplicate error code: 789
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes.rs:495: E0789: include_str!("./error_codes/E0789.md"),
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes.rs:648:     E0789, // rustc_allowed_through_unstable_modules without stability attribute
Error code E0789 has a UI test, it shouldn't be listed into EXEMPTED_FROM_TEST!
Found 1 error(s) in error codes
Done!
* 379 features
some tidy checks failed
