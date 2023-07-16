
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: broken MIR in DefId(0:3 ~ playground[f756]::main[0]) (_1 = &(*_3)): bad assignment (&impl Sized = &i32): NoSolution
 --> src/main.rs:5:9
  |
5 |     let ref _x: impl Sized = 5;
  |         ^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17

