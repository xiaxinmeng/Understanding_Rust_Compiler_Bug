
$ clang-10 -c -fPIC test.c --target=wasm32-unknown-unknown-wasm -o test.o -O3
$ wasm-ld --shared --no-entry test.o -o test.wasm --export=foo --gc-sections
wasm-ld: error: test.o: relocation R_WASM_MEMORY_ADDR_LEB cannot be used against symbol s; recompile with -fPIC
