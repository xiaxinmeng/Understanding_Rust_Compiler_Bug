
rustc --edition=2018 untrusted.rs  --crate-type lib -C codegen-units=1 -C linker-plugin-lto
rustc --edition=2018 regression.rs --crate-type bin -C codegen-units=1 -C lto -C opt-level=3 --extern untrusted=libuntrusted.rlib
