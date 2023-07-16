
rustc --crate-name rust_test src/main.rs --crate-type bin --emit=llvm-ir -C debuginfo=2 -C metadata=e19299b369c7fab4 -C extra-filename=-e19299b369c7fab4 --out-dir /scratch/rust-test/target/debug/deps -L dependency=/scratch/rust-test/target/debug/deps --extern libc=/scratch/rust-test/target/debug/deps/liblibc-4242d2235e1c985c.rlib -L native=/scratch/rust-test/target/debug/build/rust-test-1c2036192ff415b2/out -l static=myfunc
cp target/debug/deps/rust_test-e19299b369c7fab4.ll ~/rust-intel.ll
