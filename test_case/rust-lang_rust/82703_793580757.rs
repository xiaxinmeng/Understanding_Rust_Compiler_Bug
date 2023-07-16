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
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `abs`
   --> library/core/src/num/nonzero.rs:416:17
    |
416 | /                 /// Computes the absolute value of self.
417 | |                 /// See the doc for [`abs`] for overflow behaviour.
418 | |                 ///
419 | |                 /// # Example
...   |
429 | |                 /// assert_eq!(pos, neg.abs());
    | |_______________________^
...
...
584 | / nonzero_signed_operations! {
585 | |     NonZeroI8(i8) -> NonZeroU8(u8);
586 | |     NonZeroI16(i16) -> NonZeroU16(u16);
587 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
590 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation
    |
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            See the doc for [`abs`] for overflow behaviour.
    = note: no item named `abs` in scope
    = note: no item named `abs` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `overflowing_abs`
    |
    |
465 | /                 /// Computes the absolute value of self,
466 | |                 /// with overflow information, see [`overflowing_abs`].
467 | |                 ///
468 | |                 /// # Example
...   |
481 | |                 /// assert_eq!((min, true), min.overflowing_abs());
    | |_______________________^
...
...
584 | / nonzero_signed_operations! {
585 | |     NonZeroI8(i8) -> NonZeroU8(u8);
586 | |     NonZeroI16(i16) -> NonZeroU16(u16);
587 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
590 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            with overflow information, see [`overflowing_abs`].
                                            ^^^^^^^^^^^^^^^^^
    = note: no item named `overflowing_abs` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `saturating_abs`
    |
    |
494 | /                 /// Saturating absolute value, see [`saturating_abs`].
495 | |                 ///
496 | |                 /// # Example
...   |
...   |
514 | |                 /// assert_eq!(max, min_plus.saturating_abs());
    | |_______________________^
...
...
584 | / nonzero_signed_operations! {
585 | |     NonZeroI8(i8) -> NonZeroU8(u8);
586 | |     NonZeroI16(i16) -> NonZeroU16(u16);
587 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
590 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            Saturating absolute value, see [`saturating_abs`].
                                            ^^^^^^^^^^^^^^^^
    = note: no item named `saturating_abs` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: unresolved link to `wrapping_abs`
    |
    |
523 | /                 /// Wrapping absolute value, see [`wrapping_abs`].
524 | |                 ///
525 | |                 /// # Example
...   |
...   |
542 | |                 /// # assert_eq!(max, (-max).wrapping_abs());
    | |_______________________^
...
...
584 | / nonzero_signed_operations! {
585 | |     NonZeroI8(i8) -> NonZeroU8(u8);
586 | |     NonZeroI16(i16) -> NonZeroU16(u16);
587 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
590 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            Wrapping absolute value, see [`wrapping_abs`].
                                          ^^^^^^^^^^^^^^
    = note: no item named `wrapping_abs` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: unresolved link to `abs`
   --> library/core/src/num/nonzero.rs:416:17
    |
    |
416 | /                 /// Computes the absolute value of self.
417 | |                 /// See the doc for [`abs`] for overflow behaviour.
418 | |                 ///
419 | |                 /// # Example
...   |
429 | |                 /// assert_eq!(pos, neg.abs());
    | |_______________________^
...
...
584 | / nonzero_signed_operations! {
585 | |     NonZeroI8(i8) -> NonZeroU8(u8);
586 | |     NonZeroI16(i16) -> NonZeroU16(u16);
587 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
590 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            See the doc for [`abs`] for overflow behaviour.
    = note: no item named `abs` in scope
    = note: no item named `abs` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 5 previous errors

error: could not document `core`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.52.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.52.0-nightly
  (6143409c1
  2021-03-09)'` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.52.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc --stage 0 library/std
Build completed unsuccessfully in 0:00:11
