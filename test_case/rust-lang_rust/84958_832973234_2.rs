
> rm -rf ./target ; mkdir target 
> rustc --crate-name untrusted --edition=2018 src/lib.rs --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C codegen-units=1 --out-dir ./target 
> rustc --crate-name untrusted --edition=2018 src/main.rs --crate-type bin --emit=dep-info,link -C opt-level=3 -C lto -C codegen-units=1 --out-dir ./target --extern untrusted=./target/libuntrusted.rlib 
> ./target/untrusted

thread 'main' panicked at 'assertion failed: input.at_end()', src/main.rs:17:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
