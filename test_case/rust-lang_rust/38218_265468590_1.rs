
gdb --args rustc --crate-name kernel32 src/lib.rs --crate-type lib -C opt-level=3 -C metadata=2fc09ff90264f963 -C extra-filename=-2fc09ff90264f963 --out-dir ./target/release/deps --emit=dep-info,link -L dependency=./target/release/deps --extern winapi=./target/release/deps/libwinapi-ab96afe94df678e7.rlib -C target-cpu=native

(gdb) run
# once the SIGILL happens
(gdb) bt
