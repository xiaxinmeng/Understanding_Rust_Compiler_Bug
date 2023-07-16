bash
RUSTC_BOOTSTRAP=1 rustc +beta library/core/src/lib.rs --crate-type=lib --edition=2018 --cfg bootstrap --crate-name=core
RUSTC_BOOTSTRAP=1 rustc +beta ice.rs --crate-type=lib --edition=2018 --cfg bootstrap --extern core=libcore.rlib
