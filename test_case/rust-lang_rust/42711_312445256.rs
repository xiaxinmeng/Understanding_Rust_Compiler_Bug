
[00:56:50] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-staticlib-link.stage2-x86_64-unknown-linux-gnu/liblibrary.a(library.0.o): In function `asan.module_ctor':
[00:56:50] library.cgu-0.rs:(.text.asan.module_ctor+0x2): undefined reference to `__asan_init'
[00:56:50] library.cgu-0.rs:(.text.asan.module_ctor+0x7): undefined reference to `__asan_version_mismatch_check_v8'
[00:56:50] collect2: error: ld returned 1 exit status
