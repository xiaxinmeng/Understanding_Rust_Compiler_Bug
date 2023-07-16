
$ rustc b.rs --crate-type bin --extern a=liba.so
error: cannot satisfy dependencies so `std` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: aborting due to previous error
