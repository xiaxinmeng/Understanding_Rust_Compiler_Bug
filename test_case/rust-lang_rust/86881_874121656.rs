plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error[E0425]: cannot find function `lookup_line` in this scope
  |
  |
7 |     assert_eq!(lookup_line(lines, BytePos(0)), -1);


error[E0425]: cannot find function `lookup_line` in this scope
  |
  |
8 |     assert_eq!(lookup_line(lines, BytePos(3)), 0);


error[E0425]: cannot find function `lookup_line` in this scope
  |
  |
9 |     assert_eq!(lookup_line(lines, BytePos(4)), 0);


error[E0425]: cannot find function `lookup_line` in this scope
   |
   |
11 |     assert_eq!(lookup_line(lines, BytePos(16)), 0);


error[E0425]: cannot find function `lookup_line` in this scope
   |
   |
12 |     assert_eq!(lookup_line(lines, BytePos(17)), 1);


error[E0425]: cannot find function `lookup_line` in this scope
   |
   |
13 |     assert_eq!(lookup_line(lines, BytePos(18)), 1);


error[E0425]: cannot find function `lookup_line` in this scope
   |
   |
15 |     assert_eq!(lookup_line(lines, BytePos(28)), 2);


error[E0425]: cannot find function `lookup_line` in this scope
   |
   |
16 |     assert_eq!(lookup_line(lines, BytePos(29)), 2);

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0425`.
