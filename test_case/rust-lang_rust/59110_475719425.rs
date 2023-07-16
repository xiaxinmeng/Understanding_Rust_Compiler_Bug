
$ ldd obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/rustc_macros-5dcf607750efd869
        linux-vdso.so.1 (0x00007ffd633aa000)
        libtest-f4db4b8ec8713e96.so => not found
        libstd-d301c78d889cf39d.so => not found
        libc.so.6 => /lib64/libc.so.6 (0x00007fb97f848000)
        /lib64/ld-linux-x86-64.so.2 (0x00007fb97fc3c000)

$ find obj/build/x86_64-unknown-linux-gnu/stage1* -name "libtest-*.so"     
obj/build/x86_64-unknown-linux-gnu/stage1/lib/libtest-018b174132c5801e.so
obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-musl/lib/libtest-f4db4b8ec8713e96.so
obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-f4db4b8ec8713e96.so
obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-musl/release/deps/libtest-f4db4b8ec8713e96.so
obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libtest-f4db4b8ec8713e96.so

$ find obj/build/x86_64-unknown-linux-gnu/stage1* -name "libstd-*.so" 
obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-3a98e985ce6b2adf.so
obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-0397b4f55aac49d0.so
obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-d301c78d889cf39d.so
obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps/libstd-0397b4f55aac49d0.so
obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd-d301c78d889cf39d.so
