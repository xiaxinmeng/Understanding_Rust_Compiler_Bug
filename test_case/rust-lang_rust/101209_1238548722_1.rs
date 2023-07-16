console
$ cargo build
   Compiling bitflags v1.3.2
   Compiling x86 v0.51.0
   Compiling bit_field v0.10.1
   Compiling kernel v0.1.0 (/code/rust-kernel-x86_64-static-link-code-model-kernel)
   Compiling raw-cpuid v10.5.0
    Finished dev [optimized + debuginfo] target(s) in 2.04s

$ objdump -f target/x86_64-unknown-none/debug/kernel

target/x86_64-unknown-none/debug/kernel:	file format elf64-x86-64

architecture: x86_64
start address: 0xffffffff88000000

