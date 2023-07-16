
$ rustc --edition 2021 test.rs
error: invalid instruction mnemonic 'in_stp'
   |
note: instantiated into assembly here
  --> <inline asm>:13:5
   |
13 |     IN_STP                                      // 寄存器保护
   |     ^^^^^^

error: invalid instruction mnemonic 'b.eq'
   |
note: instantiated into assembly here
  --> <inline asm>:14:5
   |
14 |     b.eq .LEnc_1_process                      // 跳转
   |     ^^^^

error: invalid instruction mnemonic 'b.eq'
   |
note: instantiated into assembly here
  --> <inline asm>:15:5
   |
15 |     b.eq .LEnc_2_process                      // 跳转
   |     ^^^^

error: invalid instruction mnemonic 'b'
   |
note: instantiated into assembly here
  --> <inline asm>:16:5
   |
16 |     b .LEnc_3_process                         // 跳转
   |     ^

error: invalid instruction mnemonic 'load_hash_table'
   |
note: instantiated into assembly here
  --> <inline asm>:19:5
   |
19 |     LOAD_HASH_TABLE                            // 加载g
   |     ^^^^^^^^^^^^^^^

error: invalid instruction mnemonic 'b.le'
   |
note: instantiated into assembly here
  --> <inline asm>:20:5
   |
20 |     b.le .LEnc_end                              // 第一次64字节处理完毕，判断剩余长度
   |     ^^^^

error: invalid instruction mnemonic 'b'
   |
note: instantiated into assembly here
  --> <inline asm>:21:5
   |
21 |     b .LEnc_1_loop                            // 进入循环处理流程
   |     ^

error: invalid instruction mnemonic 'load_hash_table'
   |
note: instantiated into assembly here
  --> <inline asm>:24:5
   |
24 |     LOAD_HASH_TABLE                                                // load hash table
   |     ^^^^^^^^^^^^^^^

error: invalid instruction mnemonic 'b.le'
   |
note: instantiated into assembly here
  --> <inline asm>:25:5
   |
25 |     b.le .LEnc_end
   |     ^^^^

error: invalid instruction mnemonic 'b'
   |
note: instantiated into assembly here
  --> <inline asm>:26:5
   |
26 |     b .LEnc_2_loop
   |     ^

error: invalid instruction mnemonic 'load_hash_table'
   |
note: instantiated into assembly here
  --> <inline asm>:29:5
   |
29 |     LOAD_HASH_TABLE
   |     ^^^^^^^^^^^^^^^

error: invalid instruction mnemonic 'b.le'
   |
note: instantiated into assembly here
  --> <inline asm>:30:5
   |
30 |     b.le .LEnc_end
   |     ^^^^

error: invalid instruction mnemonic 'b'
   |
note: instantiated into assembly here
  --> <inline asm>:31:5
   |
31 |     b .LEnc_3_loop
   |     ^

error: invalid instruction mnemonic 'enc1_loop'
   |
note: instantiated into assembly here
  --> <inline asm>:34:5
   |
34 |     ENC1_LOOP                         // 处理64字节
   |     ^^^^^^^^^

error: invalid instruction mnemonic 'b.le'
   |
note: instantiated into assembly here
  --> <inline asm>:35:5
   |
35 |     b.le .LEnc_end                          // 剩余块数为0, 退出循环
   |     ^^^^

error: invalid instruction mnemonic 'b'
   |
note: instantiated into assembly here
  --> <inline asm>:36:5
   |
36 |     b .LEnc_1_loop                        // 继续循环
   |     ^

error: invalid instruction mnemonic 'enc2_loop'
   |
note: instantiated into assembly here
  --> <inline asm>:39:5
   |
39 |     ENC2_LOOP
   |     ^^^^^^^^^

error: invalid instruction mnemonic 'b.le'
   |
note: instantiated into assembly here
  --> <inline asm>:40:5
   |
40 |     b.le .LEnc_end                          // <= 0
   |     ^^^^

error: invalid instruction mnemonic 'b'
   |
note: instantiated into assembly here
  --> <inline asm>:41:5
   |
41 |     b .LEnc_2_loop
   |     ^

error: invalid instruction mnemonic 'enc3_loop'
   |
note: instantiated into assembly here
  --> <inline asm>:44:5
   |
44 |     ENC3_LOOP
   |     ^^^^^^^^^

error: invalid instruction mnemonic 'b.le'
   |
note: instantiated into assembly here
  --> <inline asm>:45:5
   |
45 |     b.le .LEnc_end                          // <= 0
   |     ^^^^

error: invalid instruction mnemonic 'b'
   |
note: instantiated into assembly here
  --> <inline asm>:46:5
   |
46 |     b .LEnc_3_loop
   |     ^

error: invalid instruction mnemonic 'hash_block'
   |
note: instantiated into assembly here
  --> <inline asm>:49:5
   |
49 |     HASH_BLOCK
   |     ^^^^^^^^^^

error: invalid instruction mnemonic 'out_stp'
   |
note: instantiated into assembly here
  --> <inline asm>:50:5
   |
50 |     OUT_STP
   |     ^^^^^^^

error: symbol 'SOME_FUNC_A' is already defined
   |
note: instantiated into assembly here
  --> <inline asm>:65:1
   |
65 | SOME_FUNC_A:
   | ^

error: invalid instruction mnemonic 'in_stp'
   |
note: instantiated into assembly here
  --> <inline asm>:66:5
   |
66 |     IN_STP                                      // 寄存器保护
   |     ^^^^^^

error: invalid instruction mnemonic 'b.eq'
   |
note: instantiated into assembly here
  --> <inline asm>:67:5
   |
67 |     b.eq .LEnc_1_process                      // 跳转1处理逻辑
   |     ^^^^

error: invalid instruction mnemonic 'b.eq'
   |
note: instantiated into assembly here
  --> <inline asm>:68:5
   |
68 |     b.eq .LEnc_2_process                      // 跳转2处理逻辑
   |     ^^^^

error: invalid instruction mnemonic 'b'
   |
note: instantiated into assembly here
  --> <inline asm>:69:5
   |
69 |     b .LEnc_3_process                         // 跳转3处理逻辑
   |     ^

error: symbol '.LEnc_1_process' is already defined
   |
note: instantiated into assembly here
  --> <inline asm>:71:1
   |
71 | .LEnc_1_process:
   | ^

error: invalid instruction mnemonic 'b'
   |
note: instantiated into assembly here
  --> <inline asm>:72:5
   |
72 |     b .LEnc_1_loop                            // 进入循环处理流程
   |     ^

error: symbol '.LEnc_1_loop' is already defined
   |
note: instantiated into assembly here
  --> <inline asm>:74:1
   |
74 | .LEnc_1_loop:
   | ^

error: invalid instruction mnemonic 'b'
   |
note: instantiated into assembly here
  --> <inline asm>:75:5
   |
75 |     b .LEnc_1_loop                        // 继续循环
   |     ^

error: aborting due to 33 previous errors
