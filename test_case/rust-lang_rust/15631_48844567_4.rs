
test.o: In function `main':
test.rs:(.text.main+0x20): undefined reference to `__morestack'
test.o: In function `rust_begin_unwind':
test.rs:(.text.rust_begin_unwind+0x20): undefined reference to `__morestack'
test.o: In function `begin_unwind::__rust_abi':
test.rs:(.text._ZN12begin_unwind10__rust_abiE+0x20): undefined reference to `__morestack'
test.o: In function `rust_eh_personality':
test.rs:(.text.rust_eh_personality+0x20): undefined reference to `__morestack'
