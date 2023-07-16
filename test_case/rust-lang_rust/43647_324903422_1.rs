
# docker run -it --rm -v $(pwd):/myrust -w /myrust base/archlinux bash
[root@6aa90f62adb2 myrust]# pacman -Sy curl file gcc
[...]
[root@6aa90f62adb2 myrust]# curl https://sh.rustup.rs -sSf | sh
[...]
[root@6aa90f62adb2 myrust]# source $HOME/.cargo/env
[root@6aa90f62adb2 myrust]# rustup target add x86_64-unknown-linux-musl
[...]
[root@6aa90f62adb2 myrust]# rustc --target x86_64-unknown-linux-musl x.rs
[root@6aa90f62adb2 myrust]# file x
x: ELF 64-bit LSB shared object, x86-64, version 1 (GNU/Linux), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=5992690cf70188d0192de6841a4c9399b37d5cc7, with debug_info, not stripped
[root@6aa90f62adb2 myrust]# ldd x
        statically linked
[root@6aa90f62adb2 myrust]# ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc --target x86_64-unknown-linux-musl x.rs
[root@6aa90f62adb2 myrust]# file x
x: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=633f517e0c5a4fea53cf0033446edf1af079d64c, with debug_info, not stripped
[root@6aa90f62adb2 myrust]# ldd x
        not a dynamic executable
