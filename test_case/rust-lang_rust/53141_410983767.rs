
[01:10:20] error[E0261]: use of undeclared lifetime name `'a`
[01:10:20]    --> tools/clippy/clippy_lints/src/write.rs:220:32
[01:10:20]     |
[01:10:20] 220 | fn check_tts(cx: &EarlyContext<'a>, tts: &ThinTokenStream, is_write: bool) -> Option<String> {
[01:10:20]     |                                ^^ undeclared lifetime
[01:10:20]
[01:10:20] error: aborting due to previous error
