plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.14s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `pointer::offset`
     |
     |
1102 |     /// The stabilized version of this intrinsic is [`pointer::offset`].
     |                                                      ^^^^^^^^^^^^^^^^^ no item named `pointer` in scope
     |
     = note: `-D broken-intra-doc-links` implied by `-D warnings`

error: unresolved link to `pointer::wrapping_offset`
     |
     |
1119 |     /// The stabilized version of this intrinsic is [`pointer::wrapping_offset`].
     |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `pointer` in scope

error: unresolved link to `slice`
    |
    |
310 | /// statically-known size, e.g., a slice [`[T]`][slice] or a [trait object],
    |                                                  ^^^^^ no item named `slice` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `slice`
    |
    |
336 | /// statically-known size, e.g., a slice [`[T]`][slice] or a [trait object],
    |                                                  ^^^^^ no item named `slice` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `slice`
    |
    |
345 | ///     - a [slice], then the length of the slice tail must be an initialized
    |              ^^^^^ no item named `slice` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `slice`
    |
    |
479 | ///     - a [slice], then the length of the slice tail must be an initialized
    |              ^^^^^ no item named `slice` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `slice::copy_from_slice`
    |
    |
868 |     /// This is similar to [`slice::copy_from_slice`].
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^ no item named `slice` in scope

error: unresolved link to `slice::clone_from_slice`
    |
    |
926 |     /// This is similar to [`slice::clone_from_slice`] but does not drop existing elements.
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `slice` in scope

error: unresolved link to `pointer`
  |
  |
3 | //! *[See also the pointer primitive types](pointer).*
  |                                             ^^^^^^^ no item named `pointer` in scope
  |
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `pointer::offset`
   |
   |
63 | //! [`offset`]: pointer::offset
   |                 ^^^^^^^^^^^^^^^ no item named `pointer` in scope

error: unresolved link to `pointer::offset`
    |
    |
378 |     ///   See the safety documentation of [`pointer::offset`].
    |                                            ^^^^^^^^^^^^^^^^^ no item named `pointer` in scope

error: unresolved link to `pointer::offset`
    |
    |
422 |     ///   See the safety documentation of [`pointer::offset`].
    |                                            ^^^^^^^^^^^^^^^^^ no item named `pointer` in scope

error: unresolved link to `pointer::offset`
    |
    |
964 |     ///   See the safety documentation of [`pointer::offset`].
    |                                            ^^^^^^^^^^^^^^^^^ no item named `pointer` in scope

error: unresolved link to `pointer::offset`
     |
     |
1224 |     ///   See the safety documentation of [`pointer::offset`].
     |                                            ^^^^^^^^^^^^^^^^^ no item named `pointer` in scope

error: unresolved link to `pointer::offset`
     |
     |
1274 |     ///   See the safety documentation of [`pointer::offset`].
     |                                            ^^^^^^^^^^^^^^^^^ no item named `pointer` in scope

error: unresolved link to `never`
    |
    |
650 | /// This enum has the same role as [the `!` “never” type][never],
    |                                                           ^^^^^ no item named `never` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to ``
   --> library/core/src/convert/mod.rs:426:15
    |
    |
426 | /// When the [`!`] type is stabilized [`Infallible`] and [`!`] will be
    |               ^^^ no item named `` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to ``
   --> library/core/src/convert/mod.rs:426:59
    |
    |
426 | /// When the [`!`] type is stabilized [`Infallible`] and [`!`] will be
    |                                                           ^^^ no item named `` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `fn`
  --> library/core/src/ops/function.rs:31:26
   |
   |
31 | /// [function pointers]: fn
   |                          ^^ no item named `fn` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `fn`
   --> library/core/src/ops/function.rs:100:26
    |
    |
100 | /// [function pointers]: fn
    |                          ^^ no item named `fn` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `fn`
   --> library/core/src/ops/function.rs:179:26
    |
    |
179 | /// [function pointers]: fn
    |                          ^^ no item named `fn` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `slice::get_unchecked`
    |
    |
746 |     /// The returned [`Range`] is safe to pass to [`slice::get_unchecked`] and
    |                                                    ^^^^^^^^^^^^^^^^^^^^^^ no item named `slice` in scope

error: unresolved link to `slice::get_unchecked_mut`
    |
    |
747 |     /// [`slice::get_unchecked_mut`] for slices of the given length.
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `slice` in scope

error: unresolved link to `array`
  |
  |
5 | //! *[See also the array primitive type](array).*
  |                                          ^^^^^ no item named `array` in scope
  |
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `array`
  --> library/core/src/array/iter.rs:11:17
   |
11 | /// A by-value [array] iterator.
   |                 ^^^^^ no item named `array` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `pointer`
   |
   |
10 | /// Equivalent to C's `void` type when used as a [pointer].
   |                                                   ^^^^^^^ no item named `pointer` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `pointer::offset`
  --> library/core/src/slice/raw.rs:34:40
   |
34 | ///   See the safety documentation of [`pointer::offset`].
   |                                        ^^^^^^^^^^^^^^^^^ no item named `pointer` in scope

error: unresolved link to `pointer::offset`
   --> library/core/src/slice/raw.rs:123:40
    |
123 | ///   See the safety documentation of [`pointer::offset`].
    |                                        ^^^^^^^^^^^^^^^^^ no item named `pointer` in scope

error: unresolved link to `slice`
   |
   |
17 | /// [byteslice]: slice
   |                  ^^^^^ no item named `slice` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `slice`
    |
    |
148 |     ///     - a [slice], then the length of the slice tail must be an intialized
    |                  ^^^^^ no item named `slice` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: aborting due to 30 previous errors

error: could not document `core`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.50.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.50.0-nightly
  (f04efe6b1
  2020-12-19)'` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.50.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc --stage 0 library/std
Build completed unsuccessfully in 0:00:10
