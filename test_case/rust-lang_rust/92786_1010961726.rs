
 Depth=1
.Ltmp169:
  .loc  25 2 9 is_stmt 1                // test.rs:2:9
  ldr d0, [sp, #104]
  mrs s0, NZCV
  str s0, [sp, #12]                   // 4-byte Folded Spill
  mrs d1, NZCV
  str d1, [sp, #128]
.Ltmp170:
  .loc  25 2 9 is_stmt 0                // test.rs:2:9
  mrs d1, NZCV
  str d1, [sp, #136]
.Ltmp171:
  .loc  25 2 14                         // test.rs:2:14
  mrs d1, NZCV
  str d1, [sp, #144]
.Ltmp172:
  .loc  25 3 20 is_stmt 1               // test.rs:3:20
  mrs x8, NZCV
  ucvtf s0, x8
  str s0, [sp, #152]
