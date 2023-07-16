
$ rustc --target arm-unknown-linux-gnueabi -C linker=arm-linux-gnueabi-gcc -Z print-link-args foo.rs
"arm-linux-gnueabi-gcc" (..) "-l" "dl" "-l" "rt" "-l" "pthread" "-l" "gcc_s" "-l" "pthread" "-l" "c" "-l" "m" "-l" "rt" "-l" "util" "-l" "compiler-rt"
