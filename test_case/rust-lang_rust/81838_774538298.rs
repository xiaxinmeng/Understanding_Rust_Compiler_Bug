rust
#![feature(global_asm)]

const BAR: u64 = 10;

global_asm!("
.global bar
bar:
  add x0, x0, {bar}
  ret
", bar = const(BAR));
