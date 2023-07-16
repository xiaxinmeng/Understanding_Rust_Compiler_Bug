sh
mkdir target/
rustc --crate-name a src/a.rs --crate-type lib --out-dir target -C metadata=-1 -C extra-filename=-1
rustc --crate-name a src/a.rs --crate-type lib --out-dir target -C metadata=-2 -C extra-filename=-2
rustc --crate-name b src/b.rs --crate-type lib --out-dir target -L dependency=target --extern a=target/liba-1.rlib
rustc --crate-name c src/c.rs --crate-type lib --out-dir target -L dependency=target --extern a=target/liba-2.rlib --extern b=target/libb.rlib
