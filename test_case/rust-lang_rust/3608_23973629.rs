
rustc -O --target i386-intel-linux --lib -o main.bc --emit-llvm main.rs
clang -ffreestanding -c main.bc -o main.o
