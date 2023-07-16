
rustc test.rs -C llvm-args="-load=`pwd`/../Enzyme/enzyme/buildrs/Enzyme/LLVMEnzyme-11.so" -C passes="enzyme"
