sh
rustc --crate-name lib lib.rs --crate-type lib --edition=2018 -C codegen-units=1 -C debuginfo=2 --out-dir out
rustc --crate-name main main.rs --crate-type bin --edition=2018 -C lto -C codegen-units=1 -C debuginfo=2 --out-dir out --extern lib=out/liblib.rlib
llvm-dwarfdump -verify out/main
