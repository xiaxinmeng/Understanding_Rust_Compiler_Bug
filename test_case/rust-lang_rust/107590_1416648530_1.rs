console
  --> library/core/benches/num/flt2dec/strategy/grisu.rs:18:9
   |
16 |     let mut buf = [MaybeUninit::new(0); MAX_SIG_DIGITS];
   |         ------- variable defined here
17 |     b.iter(|| {
   |             - inferred to be a `FnMut` closure
18 |         format_shortest(black_box(&decoded), &mut buf)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---^
   |         |                                         |
   |         |                                         variable captured here
   |         returns a reference to a captured variable which escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
