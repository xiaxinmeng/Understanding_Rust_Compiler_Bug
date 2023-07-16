
$ clang --version
clang version 10.0.0
Target: x86_64-unknown-linux-gnu
Thread model: posix
InstalledDir: /home/omer/Downloads/clang+llvm-10.0.0-x86_64-linux-gnu-ubuntu-18.04/bin

$ clang --target=wasm32-unknown-unknown -nostdlib -c -fPIC -o lib.o lib.c

$ wasm-ld --version
LLD 10.0.0

$ wasm-ld --shared --import-memory --export-all --allow-undefined lib.o -o lib.wasm
wasm-ld: error: lib.o: relocation R_WASM_MEMORY_ADDR_LEB cannot be used against symbol s; recompile with -fPIC
