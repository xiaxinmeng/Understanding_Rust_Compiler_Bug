plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 583 tests
iF..........................................F................F.......i.................. 88/583
F...............................................................F....................... 176/583
..............................F......................................................... 352/583
..............................................i.....................................F... 440/583
..............................................i.....................................F... 440/583
.................F......................i...........F................................... 528/583
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] src/test/rustdoc/array-links.rs stdout ----
---- [rustdoc] src/test/rustdoc/array-links.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/array-links/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/array-links" "--deny" "warnings" "/checkout/src/test/rustdoc/array-links.rs"
stdout: none
--- stderr -------------------------------
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 2 previous errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/cross-crate-primitive-doc.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-primitive-doc/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-primitive-doc" "--deny" "warnings" "/checkout/src/test/rustdoc/cross-crate-primitive-doc.rs" "--extern-html-root-url=primitive_doc=../" "-Z" "unstable-options"
stdout: none
--- stderr -------------------------------
error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to previous error
------------------------------------------



---- [rustdoc] src/test/rustdoc/deref-slice-core.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-slice-core/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-slice-core" "--deny" "warnings" "/checkout/src/test/rustdoc/deref-slice-core.rs"
stdout: none
--- stderr -------------------------------
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 2 previous errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/doc-notable_trait-mut_t_is_not_an_iterator.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-notable_trait-mut_t_is_not_an_iterator/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-notable_trait-mut_t_is_not_an_iterator" "--deny" "warnings" "/checkout/src/test/rustdoc/doc-notable_trait-mut_t_is_not_an_iterator.rs"
stdout: none
--- stderr -------------------------------
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 2 previous errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/include_str_cut.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/include_str_cut/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/include_str_cut" "--deny" "warnings" "/checkout/src/test/rustdoc/include_str_cut.rs"
stdout: none
--- stderr -------------------------------
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 2 previous errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-23511.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23511/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23511" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-23511.rs"
stdout: none
--- stderr -------------------------------
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 2 previous errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/no_std-primitive.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/no_std-primitive/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/no_std-primitive" "--deny" "warnings" "/checkout/src/test/rustdoc/no_std-primitive.rs"
stdout: none
--- stderr -------------------------------
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 2 previous errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/primitive/no_std.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive/no_std/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive/no_std" "--deny" "warnings" "/checkout/src/test/rustdoc/primitive/no_std.rs"
stdout: none
--- stderr -------------------------------
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 2 previous errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/slice-links.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/slice-links/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/slice-links" "--deny" "warnings" "/checkout/src/test/rustdoc/slice-links.rs"
stdout: none
--- stderr -------------------------------
error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 2 previous errors
------------------------------------------


