
rustc test.rs --crate-type bin -Lnative=. -lstatic=lib --extern dep=libdep.rlib
