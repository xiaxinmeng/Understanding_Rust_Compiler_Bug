
$ diffoscope /tmp/snippet-build-{1,2}/debug/deps/libitoa-13dadf23f55f5c52.rlib
--- /tmp/snippet-build-1/debug/deps/libitoa-13dadf23f55f5c52.rlib
+++ /tmp/snippet-build-2/debug/deps/libitoa-13dadf23f55f5c52.rlib
├── file list
│ @@ -2,14 +2,14 @@
│  ----------   0        0        0        0 1970-01-01 00:00:00.000000 //
│  ?rw-r--r--   0        0        0     8888 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa0.rcgu.o
│  ?rw-r--r--   0        0        0     6072 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa1.rcgu.o
│  ?rw-r--r--   0        0        0     4168 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa2.rcgu.o
│  ?rw-r--r--   0        0        0    69312 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa3.rcgu.o
│  ?rw-r--r--   0        0        0     4368 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa4.rcgu.o
│  ?rw-r--r--   0        0        0     5936 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa5.rcgu.o
│ -?rw-r--r--   0        0        0    24170 1970-01-01 00:00:00.000000 rust.metadata.bin
│ +?rw-r--r--   0        0        0    24171 1970-01-01 00:00:00.000000 rust.metadata.bin
│  ?rw-r--r--   0        0        0     2578 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa0.rcgu.bc.z
│  ?rw-r--r--   0        0        0     3200 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa1.rcgu.bc.z
│  ?rw-r--r--   0        0        0     2236 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa2.rcgu.bc.z
│  ?rw-r--r--   0        0        0    17086 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa3.rcgu.bc.z
│  ?rw-r--r--   0        0        0     2348 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa4.rcgu.bc.z
│  ?rw-r--r--   0        0        0     2918 1970-01-01 00:00:00.000000 itoa-13dadf23f55f5c52.itoa5.rcgu.bc.z
├── rust.metadata.bin
│ @@ -1501,11 +1501,11 @@
│  00005dc0: 5854 0000 fe54 0000 1855 0000 c055 0000  XT...T...U...U..
│  00005dd0: da55 0000 8356 0000 9d56 0000 4657 0000  .U...V...V..FW..
│  00005de0: 6057 0000 0958 0000 2358 0000 cc58 0000  `W...X..#X...X..
│  00005df0: e658 0000 8f59 0000 a959 0000 525a 0000  .X...Y...Y..RZ..
│  00005e00: 6c5a 0000 155b 0000 2f5b 0000 d85b 0000  lZ...[../[...[..
│  00005e10: f25b 0000 9b5c 0000 0469 746f 6118 7838  .[...\...itoa.x8
│  00005e20: 365f 3634 2d75 6e6b 6e6f 776e 2d6c 696e  6_64-unknown-lin
│ -00005e30: 7578 2d67 6e75 d4e0 dc83 d683 979e 4011  ux-gnu........@.
│ -00005e40: 3466 e780 3d23 b56a e700 93e9 e23c dc00  4f..=#.j.....<..
│ -00005e50: 0000 0000 0ad2 bb01 0000 0000 02c0 0184  ................
│ -00005e60: 0302 b212 0a06 5adf a101                 ......Z...
│ +00005e30: 7578 2d67 6e75 81e8 88b6 97cf a0dd e301  ux-gnu..........
│ +00005e40: 1134 66e7 803d 23b5 6ae7 0093 e9e2 3cdc  .4f..=#.j.....<.
│ +00005e50: 0000 0000 000a d2bb 0100 0000 0002 c001  ................
│ +00005e60: 8403 02b2 120a 065a dfa1 01              .......Z...
