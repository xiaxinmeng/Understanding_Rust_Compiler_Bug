 sh
$ cat hello.rs 
fn main() {
    println!("Hello, world!");
}
$ LD_LIBRARY_PATH=$HOME/rust-cross/lib $HOME/rust-cross/bin/rustc --target=arm-unknown-linux-gnueabi -C linker=/usr/bin/arm-linux-gcc hello.rs
$ file hello 
hello: ELF 32-bit LSB shared object, ARM, EABI5 version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux.so.3, for GNU/Linux 2.6.32, not stripped
$ size hello 
   text    data     bss     dec     hex filename
 115129    2452      68  117649   1cb91 hello
