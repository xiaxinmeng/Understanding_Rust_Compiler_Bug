plain
##[endgroup]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 246 tests
i................................F.F.F........................................i.........  88/246
.........................ii........i..................................

failures:


---- [mir-opt] tests/mir-opt/const_allocation2.rs stdout ----
21 }
22 
23 alloc1 (static: FOO, size: 8, align: 4) {
+     ╾─alloc23─╼ 03 00 00 00                         │ ╾──╼....
25 }
26 
26 
- alloc22 (size: 48, align: 4) {
-     0x00 │ 00 00 00 00 __ __ __ __ ╾─alloc9──╼ 00 00 00 00 │ ....░░░░╾──╼....
-     0x10 │ 00 00 00 00 __ __ __ __ ╾─alloc14─╼ 02 00 00 00 │ ....░░░░╾──╼....
-     0x20 │ 01 00 00 00 2a 00 00 00 ╾─alloc20─╼ 03 00 00 00 │ ....*...╾──╼....
+ alloc23 (size: 48, align: 4) {
+     0x00 │ 00 00 00 00 __ __ __ __ ╾─alloc10─╼ 00 00 00 00 │ ....░░░░╾──╼....
+     0x10 │ 00 00 00 00 __ __ __ __ ╾─alloc15─╼ 02 00 00 00 │ ....░░░░╾──╼....
+     0x20 │ 01 00 00 00 2a 00 00 00 ╾─alloc21─╼ 03 00 00 00 │ ....*...╾──╼....
32 
32 
- alloc9 (size: 0, align: 4) {}
+ alloc10 (size: 0, align: 4) {}
34 
- alloc14 (size: 8, align: 4) {
-     ╾─alloc12─╼ ╾─alloc13─╼                         │ ╾──╼╾──╼
+ alloc15 (size: 8, align: 4) {
+     ╾─alloc13─╼ ╾─alloc14─╼                         │ ╾──╼╾──╼
38 
38 
- alloc12 (size: 1, align: 1) {
+ alloc13 (size: 1, align: 1) {
41 }
42 


- alloc13 (size: 1, align: 1) {
+ alloc14 (size: 1, align: 1) {
45 }
46 


- alloc20 (size: 12, align: 4) {
-     ╾─a17+0x3─╼ ╾─alloc18─╼ ╾─a19+0x2─╼             │ ╾──╼╾──╼╾──╼
+ alloc21 (size: 12, align: 4) {
+     ╾─a18+0x3─╼ ╾─alloc19─╼ ╾─a20+0x2─╼             │ ╾──╼╾──╼╾──╼
50 
50 
- alloc17 (size: 4, align: 1) {
+ alloc18 (size: 4, align: 1) {
52     2a 45 15 6f                                     │ *E.o
54 


- alloc18 (size: 1, align: 1) {
+ alloc19 (size: 1, align: 1) {
56     2a                                              │ *
58 


- alloc19 (size: 4, align: 1) {
+ alloc20 (size: 4, align: 1) {
60     2a 45 15 6f                                     │ *E.o
62 


thread '[mir-opt] tests/mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_allocation2.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3604:21

---- [mir-opt] tests/mir-opt/const_allocation3.rs stdout ----
21 }
22 
22 
23 alloc1 (static: FOO, size: 4, align: 4) {
+     ╾─alloc12─╼                                     │ ╾──╼
25 }
26 
26 
- alloc11 (size: 168, align: 1) {
+ alloc12 (size: 168, align: 1) {
28     0x00 │ ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab │ ................
-     0x10 │ ab ab ab ab ab ab ab ab ab ab ab ab ╾─alloc6──╼ │ ............╾──╼
+     0x10 │ ab ab ab ab ab ab ab ab ab ab ab ab ╾─alloc7──╼ │ ............╾──╼
30     0x20 │ 01 ef cd ab 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
31     0x30 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
32     0x40 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
33     0x50 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
34     0x60 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
35     0x70 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x80 │ 00 00 00 00 00 00 00 00 00 00 ╾─alloc8──╼ 00 00 │ ..........╾──╼..
-     0x80 │ 00 00 00 00 00 00 00 00 00 00 ╾─alloc8──╼ 00 00 │ ..........╾──╼..
-     0x90 │ ╾─a9+0x63─╼ 00 00 00 00 00 00 00 00 00 00 00 00 │ ╾──╼............
+     0x80 │ 00 00 00 00 00 00 00 00 00 00 ╾─alloc9──╼ 00 00 │ ..........╾──╼..
+     0x90 │ ╾a10+0x63─╼ 00 00 00 00 00 00 00 00 00 00 00 00 │ ╾──╼............
39 }
40 


- alloc6 (size: 4, align: 4) {
+ alloc7 (size: 4, align: 4) {
42     2a 00 00 00                                     │ *...
44 

- alloc8 (fn: main)
- alloc8 (fn: main)
+ alloc9 (fn: main)
46 
- alloc9 (size: 100, align: 1) {
+ alloc10 (size: 100, align: 1) {
48     0x00 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
49     0x10 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
50     0x20 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................

thread '[mir-opt] tests/mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_allocation3.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3604:21
---- [mir-opt] tests/mir-opt/const_allocation.rs stdout ----
21 }
22 
22 
23 alloc1 (static: FOO, size: 8, align: 4) {
+     ╾─alloc19─╼ 03 00 00 00                         │ ╾──╼....
25 }
26 
26 
- alloc18 (size: 48, align: 4) {
-     0x00 │ 00 00 00 00 __ __ __ __ ╾─alloc5──╼ 00 00 00 00 │ ....░░░░╾──╼....
-     0x10 │ 00 00 00 00 __ __ __ __ ╾─alloc8──╼ 02 00 00 00 │ ....░░░░╾──╼....
-     0x20 │ 01 00 00 00 2a 00 00 00 ╾─alloc13─╼ 03 00 00 00 │ ....*...╾──╼....
+ alloc19 (size: 48, align: 4) {
+     0x00 │ 00 00 00 00 __ __ __ __ ╾─alloc6──╼ 00 00 00 00 │ ....░░░░╾──╼....
+     0x10 │ 00 00 00 00 __ __ __ __ ╾─alloc9──╼ 02 00 00 00 │ ....░░░░╾──╼....
+     0x20 │ 01 00 00 00 2a 00 00 00 ╾─alloc14─╼ 03 00 00 00 │ ....*...╾──╼....
32 
32 
- alloc5 (size: 0, align: 4) {}
+ alloc6 (size: 0, align: 4) {}
34 
- alloc8 (size: 16, align: 4) {
-     ╾─alloc9──╼ 03 00 00 00 ╾─alloc10─╼ 03 00 00 00 │ ╾──╼....╾──╼....
+ alloc9 (size: 16, align: 4) {
37 }
38 
38 
- alloc9 (size: 3, align: 1) {
+ alloc10 (size: 3, align: 1) {
40     66 6f 6f                                        │ foo
42 


- alloc10 (size: 3, align: 1) {
+ alloc11 (size: 3, align: 1) {
45 }
46 


- alloc13 (size: 24, align: 4) {
-     0x00 │ ╾─alloc14─╼ 03 00 00 00 ╾─alloc15─╼ 03 00 00 00 │ ╾──╼....╾──╼....
-     0x10 │ ╾─alloc16─╼ 04 00 00 00                         │ ╾──╼....
+ alloc14 (size: 24, align: 4) {
+     0x00 │ ╾─alloc15─╼ 03 00 00 00 ╾─alloc16─╼ 03 00 00 00 │ ╾──╼....╾──╼....
+     0x10 │ ╾─alloc17─╼ 04 00 00 00                         │ ╾──╼....
51 
51 
- alloc14 (size: 3, align: 1) {
+ alloc15 (size: 3, align: 1) {
53     6d 65 68                                        │ meh
55 


- alloc15 (size: 3, align: 1) {
+ alloc16 (size: 3, align: 1) {
57     6d 6f 70                                        │ mop
59 


- alloc16 (size: 4, align: 1) {
+ alloc17 (size: 4, align: 1) {
61     6d c3 b6 70                                     │ m..p
63 


thread '[mir-opt] tests/mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_allocation.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3604:21

failures:
    [mir-opt] tests/mir-opt/const_allocation2.rs
    [mir-opt] tests/mir-opt/const_allocation3.rs
