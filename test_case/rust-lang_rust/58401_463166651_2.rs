
error: aborting due to previous error

error: Could not compile `myproject`.

Caused by:
  process didn't exit successfully: rustc --edition=2018 --crate-name skydrawer src/main.rs --color always --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=6f031011e6caecc5 -C extra-filename=-6f031011e6caecc5 --out-dir /home/myself/myproject/target/debug/deps -C incremental=/home/myself/myproject/target/debug/incremental -L dependency=/home/myself/myproject/target/debug/deps --extern azul=/home/myself/myproject/target/debug/deps/libazul-56e0f51a55cb63d8.rlib --extern petgraph=/home/myself/myproject/target/debug/deps/libpetgraph-9e9daaac5b03e701.rlib -L native=/home/myself/myproject/target/debug/build/backtrace-sys-dda1172b4cc6618b/out -L native=/usr/lib64 -L native=/usr/lib64 -L native=/usr/lib64 -L native=/home/myself/myproject/target/debug/build/libloading-092e82fdafdc31c7/out -L native=/home/myself/myproject/target/debug/build/libloading-9bc0e843867c619c/out -L native=/home/myself/myproject/target/debug/build/tinyfiledialogs-772c43913a4f2993/out
