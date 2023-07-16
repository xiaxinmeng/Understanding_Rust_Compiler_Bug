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
---- src/option.rs - option::Option<T>::unwrap_or_default (line 842) stdout ----
error[E0599]: no method named `unwrap_or_default` found for type `{integer}` in the current scope
 --> src/option.rs:847:15
  |
8 | assert_eq!(12.unwrap_or_default(), 12);
  |               ^^^^^^^^^^^^^^^^^ method not found in `{integer}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
