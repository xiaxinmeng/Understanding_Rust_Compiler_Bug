plain
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....................................iii...........F.....
failures:

---- [assembly] src/test/assembly/x86_64-floating-point-clamp.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/x86_64-floating-point-clamp.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-C" "llvm-args=-x86-asm-syntax=intel" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/auxiliary" "--emit=asm"
stdout: none
--- stderr -------------------------------
error[E0601]: `main` function not found in crate `x86_64_floating_point_clamp`
  --> /checkout/src/test/assembly/x86_64-floating-point-clamp.rs:25:2
25 | }
25 | }
   |  ^ consider adding a `main` function to `/checkout/src/test/assembly/x86_64-floating-point-clamp.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
------------------------------------------
------------------------------------------



failures:
    [assembly] src/test/assembly/x86_64-floating-point-clamp.rs
test result: FAILED. 120 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out; finished in 0.46s

Build completed unsuccessfully in 0:10:02
