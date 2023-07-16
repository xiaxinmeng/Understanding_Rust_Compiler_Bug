
Error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/andy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "test.test0.rcgu.o" "test.test1.rcgu.o" "test.test2.rcgu.o" "-o" "test" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs" "-L" "/home/andy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "c" "-Wl,-Bstatic" "/home/andy/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-9d4b7130b0117f1c.rlib" "-Wl,-Bdynamic"
  = note: test.test0.rcgu.o: In function `_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$::unwrap::h4cfe66017ee4cfbf':
          test0-317d481089b8c8fe83113de504472633.rs:(.text._ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17h4cfe66017ee4cfbfE+0x3d): undefined reference to `_Unwind_Resume'
          test.test0.rcgu.o: In function `core::result::unwrap_failed::h8821e290e357350d':
          test0-317d481089b8c8fe83113de504472633.rs:(.text._ZN4core6result13unwrap_failed17h8821e290e357350dE+0x95): undefined reference to `_Unwind_Resume'
          collect2: error: ld returned 1 exit status
