plain
failures:

---- [ui] src/test/ui/error-codes/E0461.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/error-codes/auxiliary/wrong-target.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/auxiliary/wrong-target.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0461/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "i686-unknown-linux-gnu" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0461/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function `foo` is never used
  --> /checkout/src/test/ui/error-codes/auxiliary/wrong-target.rs:13:4
LL | fn foo() {}
   |    ^^^
   |
   = note: `#[warn(dead_code)]` on by default
   = note: `#[warn(dead_code)]` on by default

error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-Wl,--version-script=/tmp/rustc8I5Bnx/list" "-m32" "/tmp/rustc8I5Bnx/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0461/auxiliary/wrong-target.wrong_target.fb74947c-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0461/auxiliary/wrong-target.4gc4u70s3t0jok3u.rcgu.rmeta" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0461/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0461/auxiliary/libwrong_target.so" "-shared" "-Wl,-zrelro,-znow" "-Wl,--strip-debug" "-nodefaultlibs" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: cannot find crti.o: No such file or directory
           /usr/bin/ld: cannot find crtn.o: No such file or directory
           collect2: error: ld returned 1 exit status

error: aborting due to previous error; 1 warning emitted
------------------------------------------

