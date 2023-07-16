plain
...............................................iii...................................... 13024/13088
................................................................
failures:

---- [ui] src/test/ui/intrinsics/validity_invariants_of.rs#sanitized stdout ----

error in revision `sanitized`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/intrinsics/validity_invariants_of.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "sanitized" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/validity_invariants_of.sanitized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "sanitizer=memory" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/validity_invariants_of.sanitized/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/tmp/rustc90BlYu/symbols.o" "-Wl,-Bstatic" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-nightly_rt.msan.a" "-Wl,--no-whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/validity_invariants_of.sanitized/a.validity_invariants_of.f6faa112-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/validity_invariants_of.sanitized/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-lstd-d678279e1a099a18" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-f7cd8351a1acf669.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/validity_invariants_of.sanitized/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: cc: error: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-nightly_rt.msan.a: No such file or directory

error: aborting due to previous error
------------------------------------------




failures:
    [ui] src/test/ui/intrinsics/validity_invariants_of.rs#sanitized
test result: FAILED. 12972 passed; 1 failed; 115 ignored; 0 measured; 0 filtered out; finished in 153.19s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:14:09
