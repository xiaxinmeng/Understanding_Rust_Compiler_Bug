
$ xargo rustc -- -Z sanitizer=leak
error[E0460]: found possibly newer version of crate `core` which `rustc_lsan` depends on
  |
  = note: perhaps that crate needs to be recompiled?
  = note: crate `core` path #1: /home/japaric/.xargo/HOST/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-6a13577b6ee6d5ad.rlib
  = note: crate `rustc_lsan` path #1: /home/japaric/.xargo/HOST/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lsan-4445467ed8d393e4.rlib

error: aborting due to previous error

error: Could not compile `leak`.
