plain
error: internal compiler error: constant in type had an ignored error
 --> src/main.rs:3:30
  |
3 | pub struct S<const N: usize>([u8; N*0]);
  |                              ^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:356:17
