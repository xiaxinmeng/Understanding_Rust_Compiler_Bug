sh
RUSTFLAGS="--emit llvm-ir,obj" cargo build --release
../../gimli/target/debug/examples/dwarf-validate target/release/deps/dwarf_error-37590773ac18128b.o

DWARF error in target/release/deps/dwarf_error-37590773ac18128b.o: Invalid intra-unit reference in unit 0x0 from DIE 0x37ea9 to 0x352f3
etc

llc target/release/deps/dwarf_error-37590773ac18128b.ll -filetype=obj -O0 -o fail.o
../../gimli/target/debug/examples/dwarf-validate fail.o

DWARF error in fail.o: Invalid intra-unit reference in unit 0x0 from DIE 0x3bbaf to 0x38812
etc
