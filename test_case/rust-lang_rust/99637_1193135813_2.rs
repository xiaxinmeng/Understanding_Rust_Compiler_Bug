sh
rustc bar.rs --crate-type=rlib
# This succeeds
rustc foo.rs --crate-type=rlib --extern bar=libbar.rlib
