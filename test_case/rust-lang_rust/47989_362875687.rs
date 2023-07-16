
---- [run-pass] run-pass/rfc1717/library-override.rs stdout ----

error: compilation failed!
status: exit code: 101
command: "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/mark/Build/rust/src/test/run-pass/rfc1717/library-override.rs" "-L" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/run-pass/rfc1717/library-override.stage1-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-lstatic=wronglibrary:rust_test_helpers" "-L" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/run-pass/rfc1717/library-override.stage1-x86_64-unknown-linux-gnu.aux"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/run-pass/rfc1717/library-override.library_override0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/run-pass/rfc1717/library-override.library_override1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/run-pass/rfc1717/library-override.stage1-x86_64-unknown-linux-gnu" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/run-pass/rfc1717/library-override.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/test/run-pass/rfc1717/library-override.stage1-x86_64-unknown-linux-gnu.aux" "-L" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,--whole-archive" "-l" "rust_test_helpers" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "-l" "rust_test_helpers" "-Wl,--no-whole-archive" "-L" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-l" "std-57fdceb1c52b4db2" "-Wl,-Bstatic" "/home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-136e26942e0df602.rlib" "-Wl,-Bdynamic" "-l" "dl" "-l" "rt" "-l" "pthread" "-l" "pthread" "-l" "gcc_s" "-l" "c" "-l" "m" "-l" "rt" "-l" "pthread" "-l" "util" "-l" "util" "-Wl,-rpath,$ORIGIN/../../../stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/home/mark/Build/rust/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
  = note: /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/librust_test_helpers.a(rust_test_helpers.o): In function `rust_dbg_extern_identity_u32':
          rust_test_helpers.c:(.text.rust_dbg_extern_identity_u32+0x0): multiple definition of `rust_dbg_extern_identity_u32'
          /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/librust_test_helpers.a(rust_test_helpers.o):rust_test_helpers.c:(.text.rust_dbg_extern_identity_u32+0x0): first defined here
          /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/librust_test_helpers.a(rust_test_helpers.o): In function `rust_dbg_extern_identity_u64':
          rust_test_helpers.c:(.text.rust_dbg_extern_identity_u64+0x0): multiple definition of `rust_dbg_extern_identity_u64'
          /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers/librust_test_helpers.a(rust_test_helpers.o):rust_test_helpers.c:(.text.rust_dbg_extern_identity_u64+0x0): first defined here
...
