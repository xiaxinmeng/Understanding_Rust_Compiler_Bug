
$ cargo +nightly-2018-09-19 check
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s

$ cargo +nightly check
error: failed to initialized index git repository

Caused by:
  an unknown git error occurred

$ rustup toolchain list
nightly-2018-09-19-x86_64-pc-windows-gnu (default)
nightly-x86_64-pc-windows-gnu
