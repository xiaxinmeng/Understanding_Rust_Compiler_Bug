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
  |
5 | use core::mem::copy;
  |     ^^^^^^^^^^^----
  |     |          |
  |     |          help: a similar name exists in the module (notice the capitalization): `Copy`
  |     no `copy` in `mem`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
