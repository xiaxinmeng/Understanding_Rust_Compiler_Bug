sh
# Compile the program in the comment above
$ rustc foo.rs -o foo.exe

# Compile a "hello world"
$ rustc hello.rs --target asmjs-unknown-emscripten -C linker=./foo.exe
