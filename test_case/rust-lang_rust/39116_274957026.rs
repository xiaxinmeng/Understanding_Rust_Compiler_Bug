
expected stderr:
error[E0514]: found crate `std` compiled by an incompatible version of rustc
  |
  = help: please recompile that crate using this compiler (rustc 1.16.0-dev (b27d71cb1 2017-01-23))
  = note: crate `std` path #1: /home/michael/Code/rustc/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-86f66de14176b4c2.rlib compiled by "rustc 1.16.0-dev (5c73f42db 2017-01-23)"
  = note: crate `std` path #2: /home/michael/Code/rustc/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-86f66de14176b4c2.so compiled by "rustc 1.16.0-dev (5c73f42db 2017-01-23)"
  = note: crate `std` path #3: /home/michael/Code/rustc/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_unicode-b421c85d3facafd6.rlib compiled by "rustc 1.16.0-dev (5c73f42db 2017-01-23)"
  = note: crate `std` path #4: /home/michael/Code/rustc/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_shim-dac73ac8aa295bd7.rlib compiled by "rustc 1.16.0-dev (5c73f42db 2017-01-23)"

error: aborting due to previous error
