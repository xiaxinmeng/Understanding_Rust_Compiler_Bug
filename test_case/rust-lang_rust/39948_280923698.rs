
$ rustc --version
rustc 1.17.0-nightly (306035c21 2017-02-18)
$ cargo rustc -- --emit=metadata --test
   Compiling abortexample v0.1.0 (file:///home/wilfred/tmp/abortexample)
error: the linked panic runtime `panic_unwind` is not compiled with this crate's panic strategy `abort`

error: aborting due to previous error

error: Could not compile `abortexample`.

To learn more, run the command again with --verbose.
