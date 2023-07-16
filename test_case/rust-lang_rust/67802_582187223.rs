
 = note: /usr/sbin/ld: /tmp/rustcanegHw/libsyntax-5fd21376722f156b.rlib(syntax-5fd21376722f156b.20e7bp0qmy5q4tkj.rcgu.o): in function `<alloc::string::String as core::iter::traits::collect::Extend<char>>::extend::{{closure}}':
          /home/aaron/repos/rust/src/liballoc/string.rs:1797: undefined reference to `_ZN5alloc6string6String4push17h4bc615fe4aa3a523E.llvm.7902427079162915264'
          /usr/sbin/ld: /home/aaron/repos/rust/src/liballoc/string.rs:1797: undefined reference to `_ZN5alloc6string6String4push17h4bc615fe4aa3a523E.llvm.7902427079162915264'
          /usr/sbin/ld: /home/aaron/repos/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-1bedce06f54f9695.so: hidden symbol `_ZN5alloc6string6String4push17h4bc615fe4aa3a523E.llvm.7902427079162915264' isn't defined
          /usr/sbin/ld: final link failed: bad value
          clang-9: error: linker command failed with exit code 1 (use -v to see invocation)
 