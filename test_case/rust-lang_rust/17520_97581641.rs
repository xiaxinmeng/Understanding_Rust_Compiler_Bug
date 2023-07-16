
test [run-pass] run-pass/backtrace.rs ... FAILED

failures:

---- [run-pass] run-pass/backtrace.rs stdout ----

error: test run failed!
status: exit code: 101
command: x86_64-apple-darwin/test/run-pass/backtrace.stage2-arm-linux-androideabi
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'bad output: thread '<main>' panicked at 'explicit panic', /Users/tamird/src/rust/src/test/run-pass/backtrace.rs:23
stack backtrace:
   1: 0xb6c53da7 - <unknown>
   2: 0xb6c5ea8b - <unknown>
   3: 0xb6c0df37 - <unknown>
   4: 0xb6f87fb7 - <unknown>
   5: 0xb6f87d73 - <unknown>
   6: 0xb6f8b913 - <unknown>
   7: 0xb6d0dfe3 - <unknown>
   8: 0xb6d0dfbb - <unknown>
   9: 0xb6d0dfbb - <unknown>
  10: 0xb6d0dfbb - <unknown>
  11: 0xb6d0dfbb - <unknown>
  12: 0xb6d0dfbb - <unknown>
  13: 0xb6d0dfbb - <unknown>
  14: 0xb6d0dfbb - <unknown>
  15: 0xb6d0dfbb - <unknown>
  16: 0xb6d0dfbb - <unknown>
  17: 0xb6d0dfbb - <unknown>
  18: 0xb6d0dfbb - <unknown>
  19: 0xb6d0dfbb - <unknown>
  20: 0xb6d0dfbb - <unknown>
  21: 0xb6d0dfbb - <unknown>
  22: 0xb6d0dfbb - <unknown>
  23: 0xb6d0dfbb - <unknown>
  24: 0xb6d0dfbb - <unknown>
  25: 0xb6d0dfbb - <unknown>
  26: 0xb6d0dfbb - <unknown>
  27: 0xb6d0dfbb - <unknown>
  28: 0xb6d0dfbb - <unknown>
  29: 0xb6d0dfbb - <unknown>
  30: 0xb6d0dfbb - <unknown>
  31: 0xb6d0dfbb - <unknown>
  32: 0xb6d0dfbb - <unknown>
  33: 0xb6d0dfbb - <unknown>
  34: 0xb6d0dfbb - <unknown>
  35: 0xb6d0dfbb - <unknown>
  36: 0xb6d0dfbb - <unknown>
  37: 0xb6d0dfbb - <unknown>
  38: 0xb6d0dfbb - <unknown>
  39: 0xb6d0dfbb - <unknown>
  40: 0xb6d0dfbb - <unknown>
  41: 0xb6d0dfbb - <unknown>
  42: 0xb6d0dfbb - <unknown>
  43: 0xb6d0dfbb - <unknown>
  44: 0xb6d0dfbb - <unknown>
  45: 0xb6d0dfbb - <unknown>
  46: 0xb6d0dfbb - <unknown>
  47: 0xb6d0dfbb - <unknown>
  48: 0xb6d0dfbb - <unknown>
  49: 0xb6d0dfbb - <unknown>
  50: 0xb6d0dfbb - <unknown>
  51: 0xb6d0dfbb - <unknown>
  52: 0xb6d0dfbb - <unknown>
  53: 0xb6d0dfbb - <unknown>
  54: 0xb6d0dfbb - <unknown>
  55: 0xb6d0dfbb - <unknown>
  56: 0xb6d0dfbb - <unknown>
  57: 0xb6d0dfbb - <unknown>
  58: 0xb6d0dfbb - <unknown>
  59: 0xb6d0dfbb - <unknown>
  60: 0xb6d0dfbb - <unknown>
  61: 0xb6d0dfbb - <unknown>
  62: 0xb6d0dfbb - <unknown>
  63: 0xb6d0dfbb - <unknown>
  64: 0xb6d0dfbb - <unknown>
  65: 0xb6d0dfbb - <unknown>
  66: 0xb6d0dfbb - <unknown>
  67: 0xb6d0dfbb - <unknown>
  68: 0xb6d0dfbb - <unknown>
  69: 0xb6d0dfbb - <unknown>
  70: 0xb6d0dfbb - <unknown>
  71: 0xb6d0dfbb - <unknown>
  72: 0xb6d0dfbb - <unknown>
  73: 0xb6d0dfbb - <unknown>
  74: 0xb6d0dfbb - <unknown>
  75: 0xb6d0dfbb - <unknown>
  76: 0xb6d0dfbb - <unknown>
  77: 0xb6d0dfbb - <unknown>
  78: 0xb6d0dfbb - <unknown>
  79: 0xb6d0dfbb - <unknown>
  80: 0xb6d0dfbb - <unknown>
  81: 0xb6d0dfbb - <unknown>
  82: 0xb6d0dfbb - <unknown>
  83: 0xb6d0dfbb - <unknown>
  84: 0xb6d0dfbb - <unknown>
  85: 0xb6d0dfbb - <unknown>
  86: 0xb6d0dfbb - <unknown>
  87: 0xb6d0dfbb - <unknown>
  88: 0xb6d0dfbb - <unknown>
  89: 0xb6d0dfbb - <unknown>
  90: 0xb6d0dfbb - <unknown>
  91: 0xb6d0dfbb - <unknown>
  92: 0xb6d0dfbb - <unknown>
  93: 0xb6d0dfbb - <unknown>
  94: 0xb6d0dfbb - <unknown>
  95: 0xb6d0dfbb - <unknown>
  96: 0xb6d0dfbb - <unknown>
  97: 0xb6d0dfbb - <unknown>
  98: 0xb6d0dfbb - <unknown>
  99: 0xb6d0dfbb - <unknown>
  100: 0xb6d0dfbb - <unknown>
 ... <frames omitted>
', /Users/tamird/src/rust/src/test/run-pass/backtrace.rs:54

------------------------------------------

thread '[run-pass] run-pass/backtrace.rs' panicked at 'explicit panic', /Users/tamird/src/rust/src/compiletest/runtest.rs:1525
