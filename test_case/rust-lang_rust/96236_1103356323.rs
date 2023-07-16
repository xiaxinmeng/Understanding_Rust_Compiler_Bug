plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 171 tests
......................................i................................................i 88/171
.............................F.F..................i...............F................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/nll/named-lifetimes-basic.rs stdout ----
---- [mir-opt] src/test/mir-opt/nll/named-lifetimes-basic.rs stdout ----
25 | '_#2r live at {bb0[0..=1]}
26 | '_#3r live at {bb0[0..=1]}
27 | '_#4r live at {bb0[0..=1]}
- | '_#1r: '_#6r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:26: 12:27)
- | '_#1r: '_#8r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:54: 12:55)
- | '_#2r: '_#7r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:42: 12:43)
- | '_#3r: '_#9r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:66: 12:67)
- | '_#6r: '_#1r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:26: 12:27)
- | '_#7r: '_#2r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:42: 12:43)
- | '_#8r: '_#1r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:54: 12:55)
- | '_#9r: '_#3r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:66: 12:67)
+ | '_#1r: '_#6r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:26: 12:27) ($DIR/named-lifetimes-basic.rs:12:26: 12:27 (#0)
+ | '_#1r: '_#8r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:54: 12:55) ($DIR/named-lifetimes-basic.rs:12:54: 12:55 (#0)
+ | '_#2r: '_#7r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:42: 12:43) ($DIR/named-lifetimes-basic.rs:12:42: 12:43 (#0)
+ | '_#3r: '_#9r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:66: 12:67) ($DIR/named-lifetimes-basic.rs:12:66: 12:67 (#0)
+ | '_#6r: '_#1r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:26: 12:27) ($DIR/named-lifetimes-basic.rs:12:26: 12:27 (#0)
+ | '_#7r: '_#2r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:42: 12:43) ($DIR/named-lifetimes-basic.rs:12:42: 12:43 (#0)
+ | '_#8r: '_#1r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:54: 12:55) ($DIR/named-lifetimes-basic.rs:12:54: 12:55 (#0)
+ | '_#9r: '_#3r due to BoringNoLocation at All($DIR/named-lifetimes-basic.rs:12:66: 12:67) ($DIR/named-lifetimes-basic.rs:12:66: 12:67 (#0)
36 |
37 fn use_x(_1: &'_#6r mut i32, _2: &'_#7r u32, _3: &'_#8r u32, _4: &'_#9r u32) -> bool {
38     debug w => _1;                       // in scope 0 at $DIR/named-lifetimes-basic.rs:12:26: 12:27

thread '[mir-opt] src/test/mir-opt/nll/named-lifetimes-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/named_lifetimes_basic.use_x.nll.0.mir', src/tools/compiletest/src/runtest.rs:3406:25

---- [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs stdout ----
---- [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs stdout ----
18 | '_#3r live at {bb1[0]}
19 | '_#4r live at {bb1[1..=3]}
20 | '_#5r live at {bb1[4..=7], bb2[0..=2]}
- | '_#3r: '_#4r due to Assignment at Single(bb1[0])
- | '_#4r: '_#5r due to Assignment at Single(bb1[3])
+ | '_#3r: '_#4r due to Assignment at Single(bb1[0]) ($DIR/region-subtyping-basic.rs:18:13: 18:18 (#0)
+ | '_#4r: '_#5r due to Assignment at Single(bb1[3]) ($DIR/region-subtyping-basic.rs:19:13: 19:14 (#0)
24 fn main() -> () {
24 fn main() -> () {
25     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:16:11: 16:11

thread '[mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.64bit.mir', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] src/test/mir-opt/storage_ranges.rs stdout ----
---- [mir-opt] src/test/mir-opt/storage_ranges.rs stdout ----
16 | '_#1r live at {bb0[0..=22]}
17 | '_#3r live at {bb0[10]}
18 | '_#4r live at {bb0[11]}
- | '_#3r: '_#4r due to Assignment at Single(bb0[10])
+ | '_#3r: '_#4r due to Assignment at Single(bb0[10]) ($DIR/storage_ranges.rs:6:17: 6:25 (#0)
21 fn main() -> () {
21 fn main() -> () {
22     let mut _0: ();                      // return place in scope 0 at $DIR/storage_ranges.rs:3:11: 3:11

thread '[mir-opt] src/test/mir-opt/storage_ranges.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/storage_ranges.main.nll.0.mir', src/tools/compiletest/src/runtest.rs:3406:25

failures:
    [mir-opt] src/test/mir-opt/nll/named-lifetimes-basic.rs
    [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs
