
$ clang --target=wasm32-unknown-emscripten -nostdlib -c -fPIC -o lib.o lib.c
$ wasm-ld --shared --import-memory --export-all --allow-undefined lib.o -o lib.wasm
