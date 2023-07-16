rust
 Running `rustc --crate-name mybin src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=83480335185a07f8 -C extra-filename=-83480335185a07f8 --out-dir D:\cargo_temp\temp1\target\debug\deps -C incremental=D:\cargo_temp\temp1\target\debug\incremental -L dependency=D:\cargo_temp\temp1\target\debug\deps --extern mylib=D:\cargo_temp\temp1\target\debug\deps\mylib.dll`
error: cannot satisfy dependencies so `std` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `core` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `compiler_builtins` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

