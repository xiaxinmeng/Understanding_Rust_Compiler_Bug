
error: this file contains an un-closed delimiter
 --> crash-298117e3012a17b3e85cddad606b2697232cba40:2:3
  |
1 | y![
  |   - un-closed delimiter
2 | Ϥ,
  |   ^

error: macros that expand to items must be delimited with braces or followed by a semicolon
 --> crash-298117e3012a17b3e85cddad606b2697232cba40:1:3
  |
1 |   y![
  |  ___^
2 | | Ϥ,
  | |__^
thread 'rustc' panicked at 'byte index 1 is not a char boundary; it is inside 'Ϥ' (bytes 0..2) of `Ϥ,`', src/libcore/str/mod.rs:2039:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error


error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
