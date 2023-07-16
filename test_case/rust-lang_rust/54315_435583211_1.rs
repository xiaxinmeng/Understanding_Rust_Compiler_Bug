
[+] Building for riscv32 Spike emulator
       Fresh version_check v0.1.4
       Fresh libc v0.2.43
       Fresh ucd-util v0.1.1
       Fresh utf8-ranges v1.0.1
       Fresh bare-metal v0.2.3
       Fresh memchr v2.0.2
       Fresh regex-syntax v0.6.2
       Fresh riscv v0.3.0
       Fresh aho-corasick v0.6.8
       Fresh lazy_static v1.1.0
       Fresh thread_local v0.3.6
       Fresh regex v1.0.5
   Compiling diosix v2.0.0 (/data/rust/rust-lang-rust/issues/misc/diosix)
     Running `rustc --crate-name kernel src/kernel/main.rs --color never --crate-type bin --emit=dep-info,link -C opt-level=3 -C panic=abort --cfg 'feature="spike"' -C metadata=dd9000a3a945a083 -C extra-filename=-dd9000a3a945a083 --out-dir /data/rust/rust-lang-rust/issues/misc/diosix/target/riscv32imac-unknown-none-elf/release/deps --target riscv32imac-unknown-none-elf -C ar=riscv32-elf-ar -C linker=riscv32-elf-ld -L dependency=/data/rust/rust-lang-rust/issues/misc/diosix/target/riscv32imac-unknown-none-elf/release/deps -L dependency=/data/rust/rust-lang-rust/issues/misc/diosix/target/release/deps --extern riscv=/data/rust/rust-lang-rust/issues/misc/diosix/target/riscv32imac-unknown-none-elf/release/deps/libriscv-88d9c8be593b66e4.rlib -C link-arg=-Tsrc/platform/riscv32/spike/link.ld`
warning: function is never used: `serial_write`
  --> src/kernel/main.rs:39:1
   |
39 | fn serial_write(s: &str) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

error[E0668]: malformed inline assembly
  --> src/kernel/main.rs:27:9
   |
27 |         asm!("li $0,-1" :"=a2"(r) ::: "volatile");
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0668`.
error: Could not compile `diosix`.

Caused by:
  process didn't exit successfully: `rustc --crate-name kernel src/kernel/main.rs --color never --crate-type bin --emit=dep-info,link -C opt-level=3 -C panic=abort --cfg 'feature="spike"' -C metadata=dd9000a3a945a083 -C extra-filename=-dd9000a3a945a083 --out-dir /data/rust/rust-lang-rust/issues/misc/diosix/target/riscv32imac-unknown-none-elf/release/deps --target riscv32imac-unknown-none-elf -C ar=riscv32-elf-ar -C linker=riscv32-elf-ld -L dependency=/data/rust/rust-lang-rust/issues/misc/diosix/target/riscv32imac-unknown-none-elf/release/deps -L dependency=/data/rust/rust-lang-rust/issues/misc/diosix/target/release/deps --extern riscv=/data/rust/rust-lang-rust/issues/misc/diosix/target/riscv32imac-unknown-none-elf/release/deps/libriscv-88d9c8be593b66e4.rlib -C link-arg=-Tsrc/platform/riscv32/spike/link.ld` (exit code: 1)
