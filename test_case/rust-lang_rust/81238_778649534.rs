plain
normalized stderr:


expected stderr:
error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { copy_nonoverlapping(x.as_ptr(), y.as_mut_ptr(), size_of::<u8>() * SIZE) };
   |
   |
   = note: `-D clippy::size-of-in-element-count` implied by `-D warnings`
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { copy_nonoverlapping(x.as_ptr(), y.as_mut_ptr(), HALF_SIZE * size_of_val(&x[0]) * 2) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { copy(x.as_ptr(), y.as_mut_ptr(), DOUBLE_SIZE * size_of::<u8>() / 2) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { copy(x.as_ptr(), y.as_mut_ptr(), DOUBLE_SIZE / (2 / size_of::<u8>())) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
error: aborting due to 4 previous errors




diff of stderr:

-error: found a count of bytes instead of a count of elements of `T`
-   |
-   |
-LL |     unsafe { copy_nonoverlapping(x.as_ptr(), y.as_mut_ptr(), size_of::<u8>() * SIZE) };
-   |                                                              ^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = note: `-D clippy::size-of-in-element-count` implied by `-D warnings`
-   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
-
-error: found a count of bytes instead of a count of elements of `T`
-   |
-   |
-LL |     unsafe { copy_nonoverlapping(x.as_ptr(), y.as_mut_ptr(), HALF_SIZE * size_of_val(&x[0]) * 2) };
-   |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
-
-error: found a count of bytes instead of a count of elements of `T`
-   |
-   |
-LL |     unsafe { copy(x.as_ptr(), y.as_mut_ptr(), DOUBLE_SIZE * size_of::<u8>() / 2) };
-   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
-
-error: found a count of bytes instead of a count of elements of `T`
-   |
-   |
-LL |     unsafe { copy(x.as_ptr(), y.as_mut_ptr(), DOUBLE_SIZE / (2 / size_of::<u8>())) };
-   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
-error: aborting due to 4 previous errors
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/size_of_in_element_count/expressions.stderr
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base' 'size_of_in_element_count/expressions.rs'
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/size_of_in_element_count/expressions.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/size_of_in_element_count/expressions.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-45a3ec2898545ae5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e9301f8b35fd63da.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3e5755171965ca17.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-1bcf52e6f16bdb4f.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-efc540c81be69f24.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/size_of_in_element_count/expressions.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

normalized stderr:
error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { x.as_ptr().copy_to(y.as_mut_ptr(), size_of::<u8>()) };
   |
   |
   = note: `-D clippy::size-of-in-element-count` implied by `-D warnings`
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { x.as_ptr().copy_to_nonoverlapping(y.as_mut_ptr(), size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_mut_ptr().copy_from(x.as_ptr(), size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_mut_ptr().copy_from_nonoverlapping(x.as_ptr(), size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_mut_ptr().write_bytes(0u8, size_of::<u8>() * SIZE) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { write_bytes(y.as_mut_ptr(), 0u8, size_of::<u8>() * SIZE) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { swap_nonoverlapping(y.as_mut_ptr(), x.as_mut_ptr(), size_of::<u8>() * SIZE) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     slice_from_raw_parts_mut(y.as_mut_ptr(), size_of::<u8>() * SIZE);
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     slice_from_raw_parts(y.as_ptr(), size_of::<u8>() * SIZE);
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { from_raw_parts_mut(y.as_mut_ptr(), size_of::<u8>() * SIZE) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { from_raw_parts(y.as_ptr(), size_of::<u8>() * SIZE) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_mut_ptr().sub(size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     y.as_ptr().wrapping_sub(size_of::<u8>());
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_ptr().add(size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     y.as_mut_ptr().wrapping_add(size_of::<u8>());
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_ptr().offset(size_of::<u8>() as isize) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     y.as_mut_ptr().wrapping_offset(size_of::<u8>() as isize);
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
error: aborting due to 17 previous errors




expected stderr:
error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { copy_nonoverlapping::<u8>(x.as_ptr(), y.as_mut_ptr(), size_of::<u8>()) };
   |
   |
   = note: `-D clippy::size-of-in-element-count` implied by `-D warnings`
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { copy_nonoverlapping(x.as_ptr(), y.as_mut_ptr(), size_of_val(&x[0])) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { x.as_ptr().copy_to(y.as_mut_ptr(), size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { x.as_ptr().copy_to_nonoverlapping(y.as_mut_ptr(), size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_mut_ptr().copy_from(x.as_ptr(), size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_mut_ptr().copy_from_nonoverlapping(x.as_ptr(), size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { copy(x.as_ptr(), y.as_mut_ptr(), size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { copy(x.as_ptr(), y.as_mut_ptr(), size_of_val(&x[0])) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_mut_ptr().write_bytes(0u8, size_of::<u8>() * SIZE) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { write_bytes(y.as_mut_ptr(), 0u8, size_of::<u8>() * SIZE) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { swap_nonoverlapping(y.as_mut_ptr(), x.as_mut_ptr(), size_of::<u8>() * SIZE) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     slice_from_raw_parts_mut(y.as_mut_ptr(), size_of::<u8>() * SIZE);
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     slice_from_raw_parts(y.as_ptr(), size_of::<u8>() * SIZE);
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { from_raw_parts_mut(y.as_mut_ptr(), size_of::<u8>() * SIZE) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { from_raw_parts(y.as_ptr(), size_of::<u8>() * SIZE) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_mut_ptr().sub(size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     y.as_ptr().wrapping_sub(size_of::<u8>());
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_ptr().add(size_of::<u8>()) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     y.as_mut_ptr().wrapping_add(size_of::<u8>());
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     unsafe { y.as_ptr().offset(size_of::<u8>() as isize) };
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type

error: found a count of bytes instead of a count of elements of `T`
   |
   |
LL |     y.as_mut_ptr().wrapping_offset(size_of::<u8>() as isize);
   |
   |
   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
error: aborting due to 21 previous errors




diff of stderr:

 error: found a count of bytes instead of a count of elements of `T`
-   |
-   |
-LL |     unsafe { copy_nonoverlapping::<u8>(x.as_ptr(), y.as_mut_ptr(), size_of::<u8>()) };
-   |                                                                    ^^^^^^^^^^^^^^^
-   |
-   = note: `-D clippy::size-of-in-element-count` implied by `-D warnings`
-   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
-
-error: found a count of bytes instead of a count of elements of `T`
-   |
-   |
-LL |     unsafe { copy_nonoverlapping(x.as_ptr(), y.as_mut_ptr(), size_of_val(&x[0])) };
-   |                                                              ^^^^^^^^^^^^^^^^^^
-   |
-   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
-
-error: found a count of bytes instead of a count of elements of `T`
    |
    |
 LL |     unsafe { x.as_ptr().copy_to(y.as_mut_ptr(), size_of::<u8>()) };
    |
    |
+   = note: `-D clippy::size-of-in-element-count` implied by `-D warnings`
    = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
 
 error: found a count of bytes instead of a count of elements of `T`
    |
    |
 LL |     unsafe { x.as_ptr().copy_to_nonoverlapping(y.as_mut_ptr(), size_of::<u8>()) };
    |
    |
    = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
 
 error: found a count of bytes instead of a count of elements of `T`
    |
    |
 LL |     unsafe { y.as_mut_ptr().copy_from(x.as_ptr(), size_of::<u8>()) };
    |
    |
    = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
 
 error: found a count of bytes instead of a count of elements of `T`
    |
    |
 LL |     unsafe { y.as_mut_ptr().copy_from_nonoverlapping(x.as_ptr(), size_of::<u8>()) };
    |
    |
    = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
 
 error: found a count of bytes instead of a count of elements of `T`
-   |
-   |
-LL |     unsafe { copy(x.as_ptr(), y.as_mut_ptr(), size_of::<u8>()) };
-   |                                               ^^^^^^^^^^^^^^^
-   |
-   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
-
-error: found a count of bytes instead of a count of elements of `T`
-   |
-   |
-LL |     unsafe { copy(x.as_ptr(), y.as_mut_ptr(), size_of_val(&x[0])) };
-   |                                               ^^^^^^^^^^^^^^^^^^
-   |
-   = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
-
-error: found a count of bytes instead of a count of elements of `T`
    |
    |
 LL |     unsafe { y.as_mut_ptr().write_bytes(0u8, size_of::<u8>() * SIZE) };
    |
    |
    = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
 
 error: found a count of bytes instead of a count of elements of `T`
    |
    |
 LL |     unsafe { write_bytes(y.as_mut_ptr(), 0u8, size_of::<u8>() * SIZE) };
    |
    |
    = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
 
 error: found a count of bytes instead of a count of elements of `T`
    |
