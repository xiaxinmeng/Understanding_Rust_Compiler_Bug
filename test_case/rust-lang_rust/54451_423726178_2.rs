sh
# compile with:
rustc --emit=obj --crate-type=staticlib -o test.o
clang -c test.c -o ctest.o
clang ctest.o test.o
# linker error here because of now genuinely duplicate symbols
