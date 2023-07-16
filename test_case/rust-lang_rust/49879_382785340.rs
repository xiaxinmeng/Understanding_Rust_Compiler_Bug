
rustc --crate-type=staticlib lib.rs -Zcross-lang-lto -O -Ccodegen-units=1 -o libsome_rust_lib.a -Cpanic=abort
clang-6.0 -c -o main.o -O2 -flto=thin main.c
clang-6.0 -O2 -o main.exe -fuse-ld=gold -flto=thin -Wl,-plugin-opt=save-temps -Wl,-start-group -L. -lsome_rust_lib main.o -Wl,-end-group
