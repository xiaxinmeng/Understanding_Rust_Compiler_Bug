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
    |
671 |         ExitCode(code)
    |                  ^^^^ expected `u32`, found `u8`
    |
help: you can convert a `u8` to a `u32`
671 |         ExitCode(code.into())
    |                      +++++++

For more information about this error, try `rustc --explain E0308`.
