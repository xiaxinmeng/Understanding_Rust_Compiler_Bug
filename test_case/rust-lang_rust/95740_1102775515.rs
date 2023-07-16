plain
 finished in 0.340 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 138 tests
iiiiiii...iiiiiiiiii................F...F............................................... 88/138
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [assembly] src/test/assembly/asm/x86-types.rs#i686 stdout ----

error in revision `i686`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/x86-types.rs" "-Zthreads=1" "--cfg" "i686" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-types.i686/x86-types.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "i686-unknown-linux-gnu" "-C" "llvm-args=--x86-asm-syntax=intel" "-C" "target-feature=+avx512bw" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-types.i686/auxiliary" "--emit=asm"
stdout: none
--- stderr -------------------------------
error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1099 | check_reg!(k0_i8 i8 "k0" "kmovb");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1099 | check_reg!(k0_i8 i8 "k0" "kmovb");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1105 | check_reg!(k0_i16 i16 "k0" "kmovw");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1105 | check_reg!(k0_i16 i16 "k0" "kmovw");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1111 | check_reg!(k0_i32 i32 "k0" "kmovd");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1111 | check_reg!(k0_i32 i32 "k0" "kmovd");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1117 | check_reg!(k0_i64 i64 "k0" "kmovq");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1117 | check_reg!(k0_i64 i64 "k0" "kmovq");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1123 | check_reg!(k0_ptr ptr "k0" "kmovq");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1123 | check_reg!(k0_ptr ptr "k0" "kmovq");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors
error: aborting due to 10 previous errors
------------------------------------------


---- [assembly] src/test/assembly/asm/x86-types.rs#x86_64 stdout ----

error in revision `x86_64`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/x86-types.rs" "-Zthreads=1" "--cfg" "x86_64" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-types.x86_64/x86-types.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-C" "llvm-args=--x86-asm-syntax=intel" "-C" "target-feature=+avx512bw" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/x86-types.x86_64/auxiliary" "--emit=asm"
stdout: none
--- stderr -------------------------------
error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1099 | check_reg!(k0_i8 i8 "k0" "kmovb");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1099 | check_reg!(k0_i8 i8 "k0" "kmovb");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1105 | check_reg!(k0_i16 i16 "k0" "kmovw");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1105 | check_reg!(k0_i16 i16 "k0" "kmovw");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1111 | check_reg!(k0_i32 i32 "k0" "kmovd");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1111 | check_reg!(k0_i32 i32 "k0" "kmovd");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1117 | check_reg!(k0_i64 i64 "k0" "kmovq");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1117 | check_reg!(k0_i64 i64 "k0" "kmovq");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1123 | check_reg!(k0_ptr ptr "k0" "kmovq");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `kreg0` can only be used as a clobber, not as an input or output
     |
     |
282  |             asm!(concat!($mov, " ", $reg, ", ", $reg), lateout($reg) y, in($reg) x);
...
...
1123 | check_reg!(k0_ptr ptr "k0" "kmovq");
     |
     = note: this error originates in the macro `check_reg` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors
