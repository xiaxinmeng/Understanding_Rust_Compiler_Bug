 powershell
> cargo run
     Running `target\debug\day21bin.exe`
thread '<main>' panicked at 'called `Option::unwrap()` on a `None` value', ../src/libcore\option.rs:366
stack backtrace:
   0:           0x4517a7 - <unknown>
   1:           0x45915e - <unknown>
   2:           0x415d1c - <unknown>
   3:           0x41648b - <unknown>
   4:           0x44e9c2 - <unknown>
   5:           0x473cd8 - <unknown>
   6:           0x46e130 - <unknown>
   7:           0x410530 - <unknown>
   8:           0x4107df - <unknown>
   9:           0x410a9a - <unknown>
  10:           0x410dd4 - <unknown>
  11:           0x40153e - <unknown>
  12:           0x4587e8 - <unknown>
  13:           0x441da1 - <unknown>
  14:           0x4586f7 - <unknown>
  15:           0x402a0a - <unknown>
  16:           0x4013b4 - <unknown>
  17:           0x4014e7 - <unknown>
  18:     0x7ffaf70a8101 - <unknown>
Process didn't exit successfully: `target\debug\day21bin.exe` (exit code: 101)
