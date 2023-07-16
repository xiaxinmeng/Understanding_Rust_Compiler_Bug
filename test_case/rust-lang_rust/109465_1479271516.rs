plain

---- [ui] tests/ui/suggestions/issue-71394-no-from-impl.rs stdout ----
diff of stderr:

5    |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
7    = help: the following other types implement trait `From<T>`:
7    = help: the following other types implement trait `From<T>`:
-              <&'input [u8] as From<gimli::read::endian_slice::EndianSlice<'input, Endian>>>
9              <[T; LANES] as From<Simd<T, LANES>>>
10              <[bool; LANES] as From<Mask<T, LANES>>>
11    = note: required for `&[u8]` to implement `Into<&[i8]>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl/issue-71394-no-from-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-71394-no-from-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/issue-71394-no-from-impl.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&[i8]: From<&[u8]>` is not satisfied
  --> fake-test-src-base/suggestions/issue-71394-no-from-impl.rs:3:25
   |
LL |     let _: &[i8] = data.into();
   |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
   = help: the following other types implement trait `From<T>`:
   = help: the following other types implement trait `From<T>`:
             <[T; LANES] as From<Simd<T, LANES>>>
             <[bool; LANES] as From<Mask<T, LANES>>>
   = note: required for `&[u8]` to implement `Into<&[i8]>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
