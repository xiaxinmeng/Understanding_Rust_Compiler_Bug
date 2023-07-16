sh
# Note: this will wait for GDB to connect
$ cargo run --target mips-unknown-linux-gnu
   Compiling rust-102722 v0.1.0 (/root/rust-102722-mwe)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `qemu-mips -L /usr/mips-linux-gnu -g 23456 target/mips-unknown-linux-gnu/debug/rust-102722`
qemu: uncaught target signal 10 (Bus error) - core dumped
Bus error (core dumped)
