plain
   Compiling hashbrown v0.11.2
   Compiling regex v1.5.5
   Compiling measureme v10.0.0
   Compiling rand v0.8.5
error: `<` is interpreted as a start of generic arguments for `u32`, not a shift
    |
92  | / macro_rules! float_impls {
92  | / macro_rules! float_impls {
93  | |     ($ty:ident, $uty:ident, $f_scalar:ident, $u_scalar:ty,
94  | |      $fraction_bits:expr, $exponent_bias:expr) => {
95  | |         impl IntoFloat for $uty {
...   |
113 | |                 let scale = 1.0 / ((1 as $u_scalar << precision) as $f_scalar);
    | |                                                    ^^ ---------- interpreted as generic arguments
    | |                                                    not interpreted as shift
...   |
150 | |     }
151 | | }
151 | | }
    | |_- in this expansion of `float_impls!`
152 | 
153 |   float_impls! { f32, u32, f32, u32, 23, 127 }
    |
help: try shifting the cast value
    |
    |
113 |                 let scale = 1.0 / (((1) as $u_scalar << precision) as $f_scalar);
    |                                     + +

error: `<` is interpreted as a start of generic arguments for `u32`, not a shift
    |
92  | / macro_rules! float_impls {
92  | / macro_rules! float_impls {
93  | |     ($ty:ident, $uty:ident, $f_scalar:ident, $u_scalar:ty,
94  | |      $fraction_bits:expr, $exponent_bias:expr) => {
95  | |         impl IntoFloat for $uty {
...   |
128 | |                 let scale = 1.0 / ((1 as $u_scalar << precision) as $f_scalar);
    | |                                                    ^^ ---------- interpreted as generic arguments
    | |                                                    not interpreted as shift
...   |
150 | |     }
151 | | }
151 | | }
    | |_- in this expansion of `float_impls!`
152 | 
153 |   float_impls! { f32, u32, f32, u32, 23, 127 }
    |
help: try shifting the cast value
    |
    |
128 |                 let scale = 1.0 / (((1) as $u_scalar << precision) as $f_scalar);
    |                                     + +

error: `<` is interpreted as a start of generic arguments for `u64`, not a shift
    |
92  | / macro_rules! float_impls {
92  | / macro_rules! float_impls {
93  | |     ($ty:ident, $uty:ident, $f_scalar:ident, $u_scalar:ty,
94  | |      $fraction_bits:expr, $exponent_bias:expr) => {
95  | |         impl IntoFloat for $uty {
...   |
113 | |                 let scale = 1.0 / ((1 as $u_scalar << precision) as $f_scalar);
    | |                                                    ^^ ---------- interpreted as generic arguments
    | |                                                    not interpreted as shift
...   |
150 | |     }
151 | | }
151 | | }
    | |_- in this expansion of `float_impls!`
...
154 |   float_impls! { f64, u64, f64, u64, 52, 1023 }
    |
help: try shifting the cast value
    |
    |
113 |                 let scale = 1.0 / (((1) as $u_scalar << precision) as $f_scalar);
    |                                     + +

error: `<` is interpreted as a start of generic arguments for `u64`, not a shift
    |
92  | / macro_rules! float_impls {
92  | / macro_rules! float_impls {
93  | |     ($ty:ident, $uty:ident, $f_scalar:ident, $u_scalar:ty,
94  | |      $fraction_bits:expr, $exponent_bias:expr) => {
95  | |         impl IntoFloat for $uty {
...   |
128 | |                 let scale = 1.0 / ((1 as $u_scalar << precision) as $f_scalar);
    | |                                                    ^^ ---------- interpreted as generic arguments
    | |                                                    not interpreted as shift
...   |
150 | |     }
151 | | }
151 | | }
    | |_- in this expansion of `float_impls!`
...
154 |   float_impls! { f64, u64, f64, u64, 52, 1023 }
    |
help: try shifting the cast value
    |
    |
128 |                 let scale = 1.0 / (((1) as $u_scalar << precision) as $f_scalar);
    |                                     + +
   Compiling matchers v0.1.0
   Compiling crypto-common v0.1.2
   Compiling block-buffer v0.10.2
error: could not compile `rand` due to 4 previous errors
