
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', src/librustc_codegen_llvm/consts.rs:459:21
stack backtrace:
   0:        0x1063dd175 - <unknown>
   1:        0x106413e30 - <unknown>
   2:        0x1063d07cb - <unknown>
   3:        0x1063e14ba - <unknown>
   4:        0x1063e11c5 - <unknown>
   5:        0x10854d842 - <unknown>
   6:        0x1063e1cf2 - <unknown>
   7:        0x1063e175d - <unknown>
   8:        0x1063e16be - <unknown>
   9:        0x10b015de7 - <unknown>
  10:        0x10b012a0b - <unknown>
  11:        0x10af3b8a8 - <unknown>
  12:        0x10af6992d - <unknown>
  13:        0x10af3b66a - <unknown>
  14:        0x10b057425 - <unknown>
  15:        0x10b054139 - <unknown>
  16:        0x108644ef9 - <unknown>
  17:        0x1085df482 - <unknown>
  18:        0x108609469 - <unknown>
  19:        0x1085e6042 - <unknown>
  20:        0x10860dd9c - <unknown>
  21:        0x1085e5519 - <unknown>
  22:        0x1085b12dc - <unknown>
  23:        0x108524cb8 - <unknown>
  24:        0x10854eca4 - <unknown>
  25:        0x108566f28 - <unknown>
  26:        0x10850f07d - <unknown>
  27:        0x1063f0f9f - <unknown>
  28:        0x108513ef7 - <unknown>
  29:        0x1063c301e - <unknown>
  30:        0x1063efdae - <unknown>
  31:     0x7fff6f1152eb - <unknown>
  32:     0x7fff6f118249 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (084beb83e 2019-09-27) running on x86_64-apple-darwin

note: compiler flags: -C opt-level=3 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `node-replication`.
