plain
---- [ui] src/test/ui/layout/unsafe-cell-hides-niche.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/unsafe-cell-hides-niche.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/unsafe-cell-hides-niche" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/unsafe-cell-hides-niche/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/layout/unsafe-cell-hides-niche.rs:27:36
   |
   |
LL |         const _: Size::<{$size}> = Size::<{size_of::<$ty>()}>;
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `32_usize`, found `24_usize`
...
LL | check_sizes!(UnsafeCell<Vec4<N32>>: 16 => 32);
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=arm-linux-androideabi
   |
   |
   = note: expected struct `Size<32_usize>`
              found struct `Size<24_usize>`
   = note: this error originates in the macro `check_sizes` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/layout/unsafe-cell-hides-niche.rs:27:36
   |
   |
LL |         const _: Size::<{$size}> = Size::<{size_of::<$ty>()}>;
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `32_usize`, found `24_usize`
...
LL | check_sizes!(UnsafeCell<Vec4<N32>>: 16 => 32);
   |
   |
   = note: expected struct `Size<32_usize>`
              found struct `Size<24_usize>`
   = note: this error originates in the macro `check_sizes` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
