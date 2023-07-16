plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1021 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0772 (line 16070) stdout ----
error: `self` parameter is only allowed in associated functions
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:16071:16
  |
3 | fn is_cool<'a>(self: &'a (dyn Person + 'static)) -> bool {/* ... */}
  |                ^^^^ not semantically valid as function parameter
  |
  = note: associated functions are those in `impl` or `trait` definitions
error[E0405]: cannot find trait `Person` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:16071:31
  |
  |
3 | fn is_cool<'a>(self: &'a (dyn Person + 'static)) -> bool {/* ... */}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0405`.
For more information about this error, try `rustc --explain E0405`.
Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0772 (line 16086) stdout ----
error[E0405]: cannot find trait `Person` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:16087:14
  |
3 | impl<'d> dyn Person + 'd {/* ... */}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0405`.
For more information about this error, try `rustc --explain E0405`.
Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0772 (line 16076) stdout ----
error[E0405]: cannot find trait `Truthy` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:16077:23
  |
3 | fn get_is_cool<'p, R: Truthy>(person: &'p (dyn Person + 'p)) -> R {/* ... */}

error[E0405]: cannot find trait `Person` in this scope
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:16077:48
  |
  |
3 | fn get_is_cool<'p, R: Truthy>(person: &'p (dyn Person + 'p)) -> R {/* ... */}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0405`.
