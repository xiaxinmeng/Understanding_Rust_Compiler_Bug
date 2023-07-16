
rm -rf ./target ; mkdir target && rustc --crate-name untrusted --edition=2018 lib.rs --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 --out-dir ./target && rustc --crate-name untrusted --edition=2018 main.rs --crate-type bin --emit=dep-info,link -C opt-level=3 --out-dir ./target --extern untrusted=./target/libuntrusted.rlib && ./target/untrusted
