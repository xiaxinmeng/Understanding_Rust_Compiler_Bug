
  = note: /usr/lib/gcc-cross/mips64-linux-gnuabi64/9/../../../../mips64-linux-gnuabi64/bin/ld: /checkout/target/mips64-unknown-linux-gnuabi64/debug/deps/main-ab030a3be80ad042.2ubbkpxxnbb3pjbz.rcgu.o: in function `main::fn_tmpnam':
          /checkout/target/mips64-unknown-linux-gnuabi64/debug/build/libc-test-e989c0bf73fb391f/out/main.rs:14071: warning: the use of `tmpnam' is dangerous, better use `mkstemp'
          /checkout/target/mips64-unknown-linux-gnuabi64/debug/deps/main-ab030a3be80ad042.2ubbkpxxnbb3pjbz.rcgu.o: in function `main::fn_fchmod':
          /checkout/target/mips64-unknown-linux-gnuabi64/debug/build/libc-test-e989c0bf73fb391f/out/main.rs:10654:(.text._ZN4main9fn_fchmod17hf698f27b911348cdE+0x44): relocation truncated to fit: R_MIPS_GOT_DISP against `fchmod@@GLIBC_2.0'
          /checkout/target/mips64-unknown-linux-gnuabi64/debug/deps/main-ab030a3be80ad042.2ubbkpxxnbb3pjbz.rcgu.o: in function `main::fn_sigaltstack':
          /checkout/target/mips64-unknown-linux-gnuabi64/debug/build/libc-test-e989c0bf73fb391f/out/main.rs:83470:(.text._ZN4main14fn_sigaltstack17h68df0bd78d5927feE+0x44): relocation truncated to fit: R_MIPS_GOT_DISP against `sigaltstack@@GLIBC_2.0'
          collect2: error: ld returned 1 exit status
