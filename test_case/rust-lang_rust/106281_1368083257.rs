plain
---- [ui] src/test/ui/const-generics/transmute.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/transmute.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/transmute/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/transmute/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0284]: type annotations needed for `[u16; _]`
   |
   |
LL |   let _ = condense_bytes([16; 10]);
   |       ^   -------------- type must be known at this point
note: required by a bound in `condense_bytes`
  --> /checkout/src/test/ui/const-generics/transmute.rs:51:43
   |
   |
LL | fn condense_bytes<const L: usize>(v: [u8; L * 2]) -> [u16; L] {
   |                                           ^^^^^ required by this bound in `condense_bytes`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
help: consider giving this pattern a type, where the the value of const parameter `L` is specified
   |
LL |   let _: [u16; _] = condense_bytes([16; 10]);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0284`.
