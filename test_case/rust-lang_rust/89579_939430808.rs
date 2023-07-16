`
failures:

---- [ui] ui/wasm/simd-to-array-80108.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wasm/simd-to-array-80108.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wasm/simd-to-array-80108" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wasm/simd-to-array-80108/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0601]: `main` function not found in crate `simd_to_array_80108`
  --> /checkout/src/test/ui/wasm/simd-to-array-80108.rs:3:1
   |
LL | / #![feature(repr_simd)]
LL | |
LL | | // Regression test for #80108
LL | |
...  |
LL | |     }
LL | | }
   | |_^ consider adding a `main` function to `/checkout/src/test/ui/wasm/simd-to-array-80108.rs`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.

------------------------------------------



failures:
    [ui] ui/wasm/simd-to-array-80108.rs

test result: FAILED. 11631 passed; 1 failed; 637 ignored; 0 measured; 0 filtered out; finished in 100.39s
