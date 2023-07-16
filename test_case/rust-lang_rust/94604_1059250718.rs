plain
Suite(test::src/test/mir-opt) not skipped for "bootstrap::test::MirOpt" -- not in [src/tools/tidy]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 167 tests
...FFF.....F........................i.F...F........................................i................ 100/167
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/const_allocation.rs stdout ----
---- [mir-opt] mir-opt/const_allocation.rs stdout ----
19         return;                          // scope 0 at $DIR/const_allocation.rs:9:2: 9:2
21 }
- 
- 
- alloc1 (static: FOO, size: 16, align: 8) {
- }
- 
- 
- alloc18 (size: 72, align: 8) {
-     0x00 │ 00 00 00 00 __ __ __ __ ╾───────alloc5────────╼ │ ....░░░░╾──────╼
-     0x10 │ 00 00 00 00 00 00 00 00 00 00 00 00 __ __ __ __ │ ............░░░░
-     0x20 │ ╾───────alloc9────────╼ 02 00 00 00 00 00 00 00 │ ╾──────╼........
-     0x30 │ 01 00 00 00 2a 00 00 00 ╾───────alloc14───────╼ │ ....*...╾──────╼
-     0x40 │ 03 00 00 00 00 00 00 00                         │ ........
- 
- 
- alloc5 (size: 0, align: 8) {}
- 
- alloc9 (size: 32, align: 8) {
-     0x00 │ ╾───────alloc8────────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
-     0x10 │ ╾───────alloc10───────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
- 
- 
- alloc8 (size: 3, align: 1) {
-     66 6f 6f                                        │ foo
- 
- 
- alloc10 (size: 3, align: 1) {
- }
- 
- 
- alloc14 (size: 48, align: 8) {
-     0x00 │ ╾───────alloc13───────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
-     0x10 │ ╾───────alloc15───────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
-     0x20 │ ╾───────alloc16───────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........
- 
- 
- alloc13 (size: 3, align: 1) {
-     6d 65 68                                        │ meh
- 
- 
- alloc15 (size: 3, align: 1) {
-     6d 6f 70                                        │ mop
- 
- 
- alloc16 (size: 4, align: 1) {
-     6d c3 b6 70                                     │ m..p
67 


thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation.main.ConstProp.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3393:25

---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
46       bb2 (cleanup): {
47           resume;                          // scope 0 at $DIR/const-promotion-extern-static.rs:9:1: 9:45
- - }
- - 
- - 
- - alloc1 (static: Y, size: 4, align: 4) {
- -     2a 00 00 00                                     │ *...
54   
55 


thread '[mir-opt] mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_promotion_extern_static.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3393:25
---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
19         return;                          // scope 0 at $DIR/const_allocation2.rs:6:2: 6:2
21 }
- 
- 
- alloc1 (static: FOO, size: 16, align: 8) {
- }
- 
- 
- alloc28 (size: 72, align: 8) {
-     0x00 │ 00 00 00 00 __ __ __ __ ╾───────alloc13───────╼ │ ....░░░░╾──────╼
-     0x10 │ 00 00 00 00 00 00 00 00 00 00 00 00 __ __ __ __ │ ............░░░░
-     0x20 │ ╾───────alloc18───────╼ 02 00 00 00 00 00 00 00 │ ╾──────╼........
-     0x30 │ 01 00 00 00 2a 00 00 00 ╾───────alloc26───────╼ │ ....*...╾──────╼
-     0x40 │ 03 00 00 00 00 00 00 00                         │ ........
- 
- 
- alloc13 (size: 0, align: 8) {}
- 
- alloc18 (size: 16, align: 8) {
-     ╾───────alloc16───────╼ ╾───────alloc17───────╼ │ ╾──────╼╾──────╼
- 
- 
- alloc16 (size: 1, align: 1) {
- }
- 
- 
- alloc17 (size: 1, align: 1) {
- }
- 
- 
- alloc26 (size: 24, align: 8) {
-     0x00 │ ╾─────alloc22+0x3─────╼ ╾───────alloc23───────╼ │ ╾──────╼╾──────╼
-     0x10 │ ╾─────alloc25+0x2─────╼                         │ ╾──────╼
- 
- 
- alloc22 (size: 4, align: 1) {
-     2a 45 15 6f                                     │ *E.o
- 
- 
- alloc23 (size: 1, align: 1) {
-     2a                                              │ *
- 
- 
- alloc25 (size: 4, align: 1) {
-     2a 45 15 6f                                     │ *E.o
65 


thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2.main.ConstProp.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3393:25
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
19         return;                          // scope 0 at $DIR/const_allocation3.rs:6:2: 6:2
21 }
- 
- 
- alloc1 (static: FOO, size: 8, align: 8) {
- }
- 
- 
- alloc11 (size: 180, align: 1) {
-     0x00 │ ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab │ ................
-     0x10 │ ab ab ab ab ab ab ab ab ab ab ab ab ╾──alloc6── │ ............╾───
-     0x20 │ ──────────╼ 01 ef cd ab 00 00 00 00 00 00 00 00 │ ───╼............
-     0x30 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x40 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x50 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x60 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x70 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x80 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ╾──── │ ..............╾─
-     0x90 │ ─────alloc8─────╼ 00 00 ╾─────alloc9+0x63─────╼ │ ─────╼..╾──────╼
-     0xb0 │ 00 00 00 00                                     │ ....
- }
- 
- 
- alloc6 (size: 4, align: 4) {
-     2a 00 00 00                                     │ *...
- 
- alloc8 (fn: main)
- 
- 
- alloc9 (size: 100, align: 1) {
-     0x00 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x10 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x20 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x30 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x40 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x50 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x60 │ 00 00 00 00                                     │ ....
57 


thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3.main.ConstProp.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3393:25
---- [mir-opt] mir-opt/const_prop/mutable_variable_no_prop.rs stdout ----
42       }
43   }
44   
44   
-   alloc1 (static: STATIC, size: 4, align: 4) {
-       2a 00 00 00                                     │ *...
-   }
49 


thread '[mir-opt] mir-opt/const_prop/mutable_variable_no_prop.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/mutable_variable_no_prop.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3393:25
---- [mir-opt] mir-opt/const_prop/read_immutable_static.rs stdout ----
42       }
43   }
44   
44   
-   alloc1 (static: FOO, size: 1, align: 1) {
-   }
-   
49 


thread '[mir-opt] mir-opt/const_prop/read_immutable_static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/read_immutable_static.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3393:25

failures:
    [mir-opt] mir-opt/const-promotion-extern-static.rs
    [mir-opt] mir-opt/const_allocation.rs
