plain

---- [ui] src/test/ui/derives/deriving-with-repr-packed.rs stdout ----
diff of stderr:

- error: `Clone` can't be derived on this `#[repr(packed)]` struct with type or const parameters (error E0133)
+ error: `Clone` can't be derived on this `#[repr(packed)]` struct with type or const parameters
3    |
3    |
4 LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
13    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
14    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
15 
15 
- error: `PartialEq` can't be derived on this `#[repr(packed)]` struct with type or const parameters (error E0133)
+ error: `PartialEq` can't be derived on this `#[repr(packed)]` struct with type or const parameters
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
18    |
18    |
19 LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
23    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
23    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
24    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
25 
- error: `Hash` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy` (error E0133)
+ error: `Hash` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
28    |
29 LL | #[derive(Default, Hash)]

33    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
33    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
34    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)
35 
- error: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy` (error E0133)
+ error: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
38    |
39 LL | #[derive(Debug, Default)]

46 error: aborting due to 4 previous errors
46 error: aborting due to 4 previous errors
47 
48 Future incompatibility report: Future breakage diagnostic:
- error: `Clone` can't be derived on this `#[repr(packed)]` struct with type or const parameters (error E0133)
+ error: `Clone` can't be derived on this `#[repr(packed)]` struct with type or const parameters
51    |
51    |
52 LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
62    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
63 
64 Future breakage diagnostic:
64 Future breakage diagnostic:
- error: `PartialEq` can't be derived on this `#[repr(packed)]` struct with type or const parameters (error E0133)
+ error: `PartialEq` can't be derived on this `#[repr(packed)]` struct with type or const parameters
67    |
67    |
68 LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]

78    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
80 Future breakage diagnostic:
80 Future breakage diagnostic:
- error: `Hash` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy` (error E0133)
+ error: `Hash` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
83    |
84 LL | #[derive(Default, Hash)]

94    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)
94    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)
95 
96 Future breakage diagnostic:
- error: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy` (error E0133)
+ error: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
99    |
100 LL | #[derive(Debug, Default)]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-with-repr-packed/deriving-with-repr-packed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/deriving-with-repr-packed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/deriving-with-repr-packed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-with-repr-packed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-with-repr-packed/auxiliary"
stdout: none
--- stderr -------------------------------
error: `Clone` can't be derived on this `#[repr(packed)]` struct with type or const parameters
   |
   |
LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/derives/deriving-with-repr-packed.rs:1:9
   |
   |
LL | #![deny(unaligned_references)]
   |         ^^^^^^^^^^^^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error: `PartialEq` can't be derived on this `#[repr(packed)]` struct with type or const parameters
   |
   |
LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)

error: `Hash` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
   |
LL | #[derive(Default, Hash)]
   |                   ^^^^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
   |
LL | #[derive(Debug, Default)]
   |          ^^^^^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 4 previous errors

Future incompatibility report: Future breakage diagnostic:
error: `Clone` can't be derived on this `#[repr(packed)]` struct with type or const parameters
   |
   |
LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/derives/deriving-with-repr-packed.rs:1:9
   |
   |
LL | #![deny(unaligned_references)]
   |         ^^^^^^^^^^^^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

Future breakage diagnostic:
error: `PartialEq` can't be derived on this `#[repr(packed)]` struct with type or const parameters
   |
   |
LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/derives/deriving-with-repr-packed.rs:1:9
   |
   |
LL | #![deny(unaligned_references)]
   |         ^^^^^^^^^^^^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
error: `Hash` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
   |
LL | #[derive(Default, Hash)]
   |                   ^^^^
   |
---
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

Future breakage diagnostic:
error: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
   |
LL | #[derive(Debug, Default)]
   |          ^^^^^
   |
