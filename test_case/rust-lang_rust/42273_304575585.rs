
mkdir -p build/kernel
RUSTC="./krustc.sh" RUSTDOC="./krustdoc.sh" CARGO_INCREMENTAL=1 cargo rustc --manifest-path rust/src/libcollections/Cargo.toml --target x86_64-unknown-none --release -- -C soft-float -o build/kernel/libcollections.rlib
   Compiling core v0.0.0 (file:///home/ahmed/code/rust/redox/rust/src/libcore)
error: unused macro definition
  --> rust/src/libcore/num/float_macros.rs:13:1
   |
13 | / macro_rules! assert_approx_eq {
14 | |     ($a:expr, $b:expr) => ({
15 | |         use num::Float;
16 | |         let (a, b) = (&$a, &$b);
...  |
19 | |     })
20 | | }
   | |_^
   |
   = note: #[deny(unused_macros)] implied by #[deny(warnings)]
note: lint level defined here
  --> rust/src/libcore/lib.rs:68:9
   |
68 | #![deny(warnings)]
   |         ^^^^^^^^

error: unused macro definition
  --> rust/src/libcore/num/wrapping.rs:15:1
   |
15 | / macro_rules! sh_impl_signed {
16 | |     ($t:ident, $f:ident) => (
17 | |         #[stable(feature = "rust1", since = "1.0.0")]
18 | |         impl Shl<$f> for Wrapping<$t> {
...  |
60 | |     )
61 | | }
   | |_^
   |
   = note: #[deny(unused_macros)] implied by #[deny(warnings)]

error: unused macro definition
   --> rust/src/libcore/num/mod.rs:99:1
    |
99  | / macro_rules! checked_op {
100 | |     ($U:ty, $op:path, $x:expr, $y:expr) => {{
101 | |         let (result, overflowed) = unsafe { $op($x as $U, $y as $U) };
102 | |         if overflowed { None } else { Some(result as Self) }
103 | |     }}
104 | | }
    | |_^
    |
    = note: #[deny(unused_macros)] implied by #[deny(warnings)]

error: unused macro definition
   --> rust/src/libcore/ops.rs:766:1
    |
766 | / macro_rules! neg_impl_unsigned {
767 | |     ($($t:ty)*) => {
768 | |         neg_impl_core!{ x => {
769 | |             !x.wrapping_add(1)
770 | |         }, $($t)*} }
771 | | }
    | |_^
    |
    = note: #[deny(unused_macros)] implied by #[deny(warnings)]

error: internal compiler error: /checkout/src/librustc_typeck/check/mod.rs:3295: unexpected definition: Enum(DefId { krate: CrateNum(0), node: DefIndex(13457) => core/912dd31c2582a9b9a1af69a0b75b641c::ops[0]::RangeInclusive[0] })
