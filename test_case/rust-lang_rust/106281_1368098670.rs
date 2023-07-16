plain
---- [ui] src/test/ui/const-generics/transmute.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/transmute.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/transmute/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/transmute/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/transmute.rs:71:36
   |
   |
LL |   let _: [u8; 5] = condense_bytes([16u16; 10]);
   |                                    ^^^^^ expected `u8`, found `u16`
   |
help: change the type of the numeric literal from `u16` to `u8`
   |
LL |   let _: [u8; 5] = condense_bytes([16u8; 10]);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/transmute.rs:71:20
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL |   let _: [u8; 5] = condense_bytes([16u16; 10]);
   |          -------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u8`, found `u16`
   |          expected due to this
   |
   |
   = note: expected array `[u8; 5]`
              found array `[u16; _]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
