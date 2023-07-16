
[00:02:54] error[E0308]: mismatched types
[00:02:54]     --> /checkout/src/libstd/f32.rs:1143:17
[00:02:54]      |
[00:02:54] 1143 |             v = NAN;
[00:02:54]      |                 ^^^ expected u32, found f32
[00:02:54]      |
[00:02:54]      = help: here are some functions which might fulfill your needs:
[00:02:54]              - .to_bits()
[00:02:54] 
[00:02:54] error[E0308]: mismatched types
[00:02:54]     --> /checkout/src/libstd/f64.rs:1058:17
[00:02:54]      |
[00:02:54] 1058 |             v = NAN;
[00:02:54]      |                 ^^^ expected u64, found f64
[00:02:54]      |
[00:02:54]      = help: here are some functions which might fulfill your needs:
[00:02:54]              - .to_bits()
[00:02:54] 
[00:02:55] error: aborting due to previous error(s)
