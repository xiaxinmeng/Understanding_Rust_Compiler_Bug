plain
........................................................................................  4136/14886
........................................................................................  4224/14886
........................................................................................  4312/14886
........................................................................i..........i....  4400/14886
......i.....................................F.F.........................................  4488/14886
........................F.............................................iii...............  4576/14886
..............................................................i.........................  4752/14886
........................................................................................  4840/14886
........................................................................................  4928/14886
........................................................................................  5016/14886
---
........................................................................................ 10648/14886
........................................................................................ 10736/14886
.........................................................FF............................. 10824/14886
........................................................................................ 10912/14886
.......................ii.F.............i.....iii........F............F................. 11000/14886
........................................................................................ 11176/14886
........................................................................................ 11264/14886
........................................................................................ 11352/14886
........................................................................................ 11440/14886
---
........................................................................................ 14256/14886
........................................................................................ 14344/14886
........................................................................................ 14432/14886
........................................................................................ 14520/14886
.................F.FF.FF................................................................ 14608/14886
.............................................................................iii........ 14784/14886
........................................................................................ 14872/14886
..............


failures:

---- [ui] tests/ui/asm/unpretty-expanded.rs stdout ----
diff of stdout:

1 #![feature(prelude_import)]
2 #![no_std]
+ // needs-asm-support
+ // check-pass
+ // compile-flags: -Zunpretty=expanded
+ #![feature(internal_features_lint)]
3 #[prelude_import]
4 use ::std::prelude::rust_2015::*;

6 extern crate std;
- // needs-asm-support
- // check-pass
- // check-pass
- // compile-flags: -Zunpretty=expanded
10 global_asm! ("x: .byte 42");


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/unpretty-expanded/unpretty-expanded.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/unpretty-expanded/unpretty-expanded.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/unpretty-expanded.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/asm/unpretty-expanded.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/unpretty-expanded" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/unpretty-expanded/auxiliary" "-Zunpretty=expanded"
#![feature(prelude_import)]
#![no_std]
// needs-asm-support
// check-pass
// check-pass
// compile-flags: -Zunpretty=expanded
#![feature(internal_features_lint)]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
extern crate std;
global_asm! ("x: .byte 42");
stderr: none


---- [ui] tests/ui/associated-type-bounds/return-type-notation/unpretty-parenthesized.rs stdout ----
---- [ui] tests/ui/associated-type-bounds/return-type-notation/unpretty-parenthesized.rs stdout ----
diff of stdout:

1 #![feature(prelude_import)]
+ // edition: 2021
+ // compile-flags: -Zunpretty=expanded
+ #![feature(internal_features_lint)]
2 #[prelude_import]
3 use std::prelude::rust_2021::*;
4 #[macro_use]
4 #[macro_use]

5 extern crate std;
- // edition: 2021
- // compile-flags: -Zunpretty=expanded
9 trait Trait {
10     async fn method() {}
11 }


- 
13 fn foo<T: Trait<method(i32) : Send>>() {}
15 fn main() {}
16 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/return-type-notation/unpretty-parenthesized/unpretty-parenthesized.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-type-bounds/return-type-notation/unpretty-parenthesized.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-type-bounds/return-type-notation/unpretty-parenthesized.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/return-type-notation/unpretty-parenthesized" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/return-type-notation/unpretty-parenthesized/auxiliary" "--edition=2021" "-Zunpretty=expanded"
#![feature(prelude_import)]
// edition: 2021
// edition: 2021
// compile-flags: -Zunpretty=expanded

//~^ ERROR associated type bounds are unstable

#![feature(internal_features_lint)]
#![feature(internal_features_lint)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
trait Trait {
    async fn method() {}
}
fn foo<T: Trait<method(i32) : Send>>() {}
------------------------------------------
--- stderr -------------------------------
error[E0658]: associated type bounds are unstable
  --> fake-test-src-base/associated-type-bounds/return-type-notation/unpretty-parenthesized.rs:8:17
  --> fake-test-src-base/associated-type-bounds/return-type-notation/unpretty-parenthesized.rs:8:17
   |
LL | fn foo<T: Trait<method(i32): Send>>() {}
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
---
To only update this specific test, also pass `--test-args cfg/cfg-false-feature.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/cfg/cfg-false-feature.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/cfg-false-feature" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/cfg-false-feature/auxiliary" "--crate-type" "lib"
stdout: none
warning: unknown lint: `internal_features`
   |
   = note: the `internal_features` lint is unstable
   = help: add `#![feature(internal_features_lint)]` to the crate attributes to enable
---

warning: trait aliases are experimental
  --> fake-test-src-base/cfg/cfg-false-feature.rs:14:1
   |
LL | trait A = Clone; //~ WARN trait aliases are experimental
   |
   = note: see issue #41517 <https://github.com/rust-lang/rust/issues/41517> for more information
   = help: add `#![feature(trait_alias)]` to the crate attributes to enable
   = warning: unstable syntax can change at any point in the future, causing a hard error!
   = warning: unstable syntax can change at any point in the future, causing a hard error!
   = note: for more information, see issue #65860 <https://github.com/rust-lang/rust/issues/65860>

warning: `macro` is experimental
  --> fake-test-src-base/cfg/cfg-false-feature.rs:11:1
   |
LL | macro mac() {} //~ WARN `macro` is experimental
   |
   = note: see issue #39412 <https://github.com/rust-lang/rust/issues/39412> for more information
   = note: see issue #39412 <https://github.com/rust-lang/rust/issues/39412> for more information
   = help: add `#![feature(decl_macro)]` to the crate attributes to enable
   = note: for more information, see issue #65860 <https://github.com/rust-lang/rust/issues/65860>

warning: box pattern syntax is experimental
  --> fake-test-src-base/cfg/cfg-false-feature.rs:18:9
  --> fake-test-src-base/cfg/cfg-false-feature.rs:18:9
   |
LL |     let box _ = Box::new(0); //~ WARN box pattern syntax is experimental
   |
   = note: see issue #29641 <https://github.com/rust-lang/rust/issues/29641> for more information
   = help: add `#![feature(box_patterns)]` to the crate attributes to enable
   = warning: unstable syntax can change at any point in the future, causing a hard error!
---

---- [ui] tests/ui/codemap_tests/unicode.rs#expanded stdout ----
diff of stdout:

1 #![feature(prelude_import)]
2 #![no_std]
- #[prelude_import]
- use ::std::prelude::rust_2015::*;
- #[macro_use]
- extern crate std;
7 // revisions: normal expanded
8 //[expanded] check-pass
9 //[expanded]compile-flags: -Zunpretty=expanded
10 
+ #![feature(internal_features_lint)]
+ #![feature(internal_features_lint)]
+ #[prelude_import]
+ use ::std::prelude::rust_2015::*;
+ #[macro_use]
+ extern crate std;
11 extern "路濫狼á́́" fn foo() {}
13 fn main() {}
14 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode.expanded/unicode.expanded.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args codemap_tests/unicode.rs`

error in revision `expanded`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/codemap_tests/unicode.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "expanded" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode.expanded" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode.expanded/auxiliary" "-Zunpretty=expanded"
#![feature(prelude_import)]
#![no_std]
// revisions: normal expanded
//[expanded] check-pass
//[expanded] check-pass
//[expanded]compile-flags: -Zunpretty=expanded

//[normal]~ ERROR invalid ABI
#![feature(internal_features_lint)]
#[prelude_import]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
extern "路濫狼á́́" fn foo() {}
fn main() {}
------------------------------------------
---
1 error[E0601]: `main` function not found in crate `cfg_in_crate_1`
2   --> $DIR/cfg-in-crate-1.rs:3:13
3    |

4 LL | #![cfg(bar)]
5    |             ^ consider adding a `main` function to `$DIR/cfg-in-crate-1.rs`
- error: aborting due to previous error
+ error: aborting due to previous error; 2 warnings emitted
8 
9 For more information about this error, try `rustc --explain E0601`.
9 For more information about this error, try `rustc --explain E0601`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-in-crate-1/cfg-in-crate-1.stderr
To only update this specific test, also pass `--test-args conditional-compilation/cfg-in-crate-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/conditional-compilation/cfg-in-crate-1.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-in-crate-1" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-in-crate-1/auxiliary"
stdout: none
warning: unknown lint: `internal_features`
   |
   = note: the `internal_features` lint is unstable
   = help: add `#![feature(internal_features_lint)]` to the crate attributes to enable
---

error[E0601]: `main` function not found in crate `cfg_in_crate_1`
  --> fake-test-src-base/conditional-compilation/cfg-in-crate-1.rs:3:13
   |
LL | #![cfg(bar)]
   |             ^ consider adding a `main` function to `fake-test-src-base/conditional-compilation/cfg-in-crate-1.rs`
error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0601`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/const-generics/defaults/pretty-printing-ast.rs stdout ----
diff of stdout:

5 // compile-flags: -Z unpretty=expanded
7 #![crate_type = "lib"]
+ 
+ 
+ 
+ 
+ #![feature(internal_features_lint)]
8 #[prelude_import]
9 use ::std::prelude::rust_2015::*;

11 extern crate std;
- 
- 
13 trait Foo<const KIND : bool = true> {}
- 
15 fn foo<const SIZE : usize = 5>() {}
- 
17 struct Range<const FROM : usize = 0, const LEN : usize = 0, const TO : usize =
18     FROM>;


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/pretty-printing-ast/pretty-printing-ast.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/pretty-printing-ast/pretty-printing-ast.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/defaults/pretty-printing-ast.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/defaults/pretty-printing-ast.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/pretty-printing-ast" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/pretty-printing-ast/auxiliary" "-Z" "unpretty=expanded"
#![feature(prelude_import)]
#![no_std]
#![no_std]
// Test the AST pretty printer correctly handles default values for const generics
// check-pass
// compile-flags: -Z unpretty=expanded
#![crate_type = "lib"]




#![feature(internal_features_lint)]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
extern crate std;
trait Foo<const KIND : bool = true> {}
fn foo<const SIZE : usize = 5>() {}
struct Range<const FROM : usize = 0, const LEN : usize = 0, const TO : usize =
    FROM>;
stderr: none


---- [ui] tests/ui/deriving/deriving-all-codegen.rs stdout ----
---
19 #![allow(deprecated)]
+ 
+ // Empty struct.
+ 
+ // A basic struct. Note: because this derives `Copy`, it gets the simple
+ // `clone` implemention that just does `*self`.
+ 
+ // A basic packed struct. Note: because this derives `Copy`, it gets the simple
+ // `clone` implemention that just does `*self`.
+ 
+ // A large struct. Note: because this derives `Copy`, it gets the simple
+ // `clone` implemention that just does `*self`.
+ 
+ // A struct that doesn't impl `Copy`, which means it gets the non-simple
+ // `clone` implemention that clones the fields individually.
+ 
+ // A packed struct that doesn't impl `Copy`, which means it gets the non-simple
+ // `clone` implemention that clones the fields individually.
+ 
+ // A struct that impls `Copy` manually, which means it gets the non-simple
+ // `clone` implemention that clones the fields individually.
+ 
+ // A packed struct that impls `Copy` manually, which means it gets the
+ // non-simple `clone` implemention that clones the fields individually.
+ 
+ // A struct with an unsized field. Some derives are not usable in this case.
+ 
+ // A packed struct with an unsized `[u8]` field. This is currently allowed, but
+ // causes a warning and will be phased out at some point.
+ 
+ // A generic struct involving an associated type.
+ 
+ 
+ // A packed, generic tuple struct involving an associated type. Because it is
+ // packed, a `T: Copy` bound is added to all impls (and where clauses within
+ // them) except for `Default`. This is because we must access fields using
+ // copies (e.g. `&{self.0}`), instead of using direct references (e.g.
+ // `&self.0`) which may be misaligned in a packed struct.
+ // An empty enum.
+ 
+ // A single-variant enum.
+ 
+ 
+ // A C-like, fieldless enum with a single variant.
+ 
+ // A C-like, fieldless enum.
+ 
+ // An enum with multiple fieldless and fielded variants.
+ 
+ // An enum with no fieldless variants. Note that `Default` cannot be derived
+ // for this enum.
+ 
+ // A generic enum. Note that `Default` cannot be derived for this enum.
+ 
+ // A union. Most builtin traits are not derivable for unions.
+ #![feature(internal_features_lint)]
20 #[prelude_import]
21 use std::prelude::rust_2021::*;

23 extern crate std;
- 
- // Empty struct.
- // Empty struct.
26 struct Empty;
27 #[automatically_derived]
28 impl ::core::clone::Clone for Empty {

77         ::core::cmp::Ordering::Equal
79 }
- 
- 
- // A basic struct. Note: because this derives `Copy`, it gets the simple
- // `clone` implemention that just does `*self`.
84     x: u32,
85     y: u32,

161         }
161         }
162     }
163 }
- 
- // A basic packed struct. Note: because this derives `Copy`, it gets the simple
- // `clone` implemention that just does `*self`.
167 #[repr(packed)]
168 struct PackedPoint {

248         }
249     }
250 }
250 }
- 
- // A large struct. Note: because this derives `Copy`, it gets the simple
- // `clone` implemention that just does `*self`.
254 struct Big {
255     b1: u32,
256     b2: u32,
418         }
419     }
420 }
- 
- 
- // A struct that doesn't impl `Copy`, which means it gets the non-simple
- // `clone` implemention that clones the fields individually.
424 struct NonCopy(u32);
425 #[automatically_derived]
426 impl ::core::clone::Clone for NonCopy {

429         NonCopy(::core::clone::Clone::clone(&self.0))
431 }
- 
- 
- // A packed struct that doesn't impl `Copy`, which means it gets the non-simple
- // `clone` implemention that clones the fields individually.
435 #[repr(packed)]
436 struct PackedNonCopy(u32);
437 #[automatically_derived]

441         PackedNonCopy(::core::clone::Clone::clone(&{ self.0 }))
443 }
- 
- 
- // A struct that impls `Copy` manually, which means it gets the non-simple
- // `clone` implemention that clones the fields individually.
447 struct ManualCopy(u32);
448 #[automatically_derived]
449 impl ::core::clone::Clone for ManualCopy {
453     }
454 }
455 impl Copy for ManualCopy {}
- 
- 
- // A packed struct that impls `Copy` manually, which means it gets the
- // non-simple `clone` implemention that clones the fields individually.
459 #[repr(packed)]
460 struct PackedManualCopy(u32);
461 #[automatically_derived]
466     }
467 }
467 }
468 impl Copy for PackedManualCopy {}
- 
- // A struct with an unsized field. Some derives are not usable in this case.
471 struct Unsized([u32]);
472 #[automatically_derived]
473 impl ::core::fmt::Debug for Unsized {

515         ::core::cmp::Ord::cmp(&self.0, &other.0)
517 }
- 
- 
- // A packed struct with an unsized `[u8]` field. This is currently allowed, but
- // causes a warning and will be phased out at some point.
521 #[repr(packed)]
522 struct PackedUnsizedU8([u8]);
523 #[automatically_derived]

533         ::core::hash::Hash::hash(&self.0, state)
535 }
- 
537 trait Trait {
538     type A;
538     type A;
539 }

- 
- // A generic struct involving an associated type.
542 struct Generic<T: Trait, U> {
543     t: T,
544     ta: T::A,
650         }
651     }
652 }
- 
- 
- // A packed, generic tuple struct involving an associated type. Because it is
- // packed, a `T: Copy` bound is added to all impls (and where clauses within
- // them) except for `Default`. This is because we must access fields using
- // copies (e.g. `&{self.0}`), instead of using direct references (e.g.
- // `&self.0`) which may be misaligned in a packed struct.
659 #[repr(packed)]
660 struct PackedGeneric<T: Trait, U>(T, T::A, U);
661 #[automatically_derived]
777         }
778     }
779 }
- 
- 
- // An empty enum.
782 enum Enum0 {}
783 #[automatically_derived]
784 impl ::core::clone::Clone for Enum0 {

832         unsafe { ::core::intrinsics::unreachable() }
834 }
- 
- // A single-variant enum.
837 enum Enum1 {
837 enum Enum1 {
838     Single {
839         x: u32,

912         }
913     }
914 }
- 
- // A C-like, fieldless enum with a single variant.
917 enum Fieldless1 {
919     #[default]


970         ::core::cmp::Ordering::Equal
972 }
- 
- 
- // A C-like, fieldless enum.
975 enum Fieldless {
977     #[default]


1048         ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
1050 }
- 
- 
- // An enum with multiple fieldless and fielded variants.
1053 enum Mixed {
1055     #[default]

1188         }
1189     }
1189     }
1190 }
- 
- // An enum with no fieldless variants. Note that `Default` cannot be derived
- // for this enum.
1194 enum Fielded { X(u32), Y(bool), Z(Option<i32>), }
1195 #[automatically_derived]
1196 impl ::core::clone::Clone for Fielded {
1308         }
1309     }
1310 }
- 
- 
- // A generic enum. Note that `Default` cannot be derived for this enum.
1313 enum EnumGeneric<T, U> { One(T), Two(U), }
1314 #[automatically_derived]
1315 impl<T: ::core::clone::Clone, U: ::core::clone::Clone> ::core::clone::Clone
1427         }
1428     }
1429 }
- 
- 
- // A union. Most builtin traits are not derivable for unions.
1433     pub b: bool,
1434     pub u: u32,



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-all-codegen/deriving-all-codegen.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args deriving/deriving-all-codegen.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/deriving/deriving-all-codegen.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-all-codegen" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-all-codegen/auxiliary" "-Zunpretty=expanded" "--edition=2021"
#![feature(prelude_import)]
// check-pass
// check-pass
// compile-flags: -Zunpretty=expanded
// edition:2021
//
// This test checks the code generated for all[*] the builtin derivable traits
// on a variety of structs and enums. It protects against accidental changes to
// the generated code, and makes deliberate changes to the generated code
// easier to review.
//
// [*] It excludes `Copy` in some cases, because that changes the code
// generated for `Clone`.
//
// [*] It excludes `RustcEncodable` and `RustDecodable`, which are obsolete and
// also require the `rustc_serialize` crate.
#![crate_type = "lib"]
#![allow(dead_code)]
#![allow(deprecated)]


// Empty struct.

// A basic struct. Note: because this derives `Copy`, it gets the simple
// `clone` implemention that just does `*self`.

// A basic packed struct. Note: because this derives `Copy`, it gets the simple
// `clone` implemention that just does `*self`.

// A large struct. Note: because this derives `Copy`, it gets the simple
// `clone` implemention that just does `*self`.

// A struct that doesn't impl `Copy`, which means it gets the non-simple
// `clone` implemention that clones the fields individually.

// A packed struct that doesn't impl `Copy`, which means it gets the non-simple
// `clone` implemention that clones the fields individually.

// A struct that impls `Copy` manually, which means it gets the non-simple
// `clone` implemention that clones the fields individually.

// A packed struct that impls `Copy` manually, which means it gets the
// non-simple `clone` implemention that clones the fields individually.
// A struct with an unsized field. Some derives are not usable in this case.


// A packed struct with an unsized `[u8]` field. This is currently allowed, but
// causes a warning and will be phased out at some point.
//~^ WARNING byte slice in a packed struct that derives a built-in trait
//~^^ WARNING byte slice in a packed struct that derives a built-in trait
//~^^^ this was previously accepted
//~^^^^ this was previously accepted

// A generic struct involving an associated type.


// A packed, generic tuple struct involving an associated type. Because it is
// packed, a `T: Copy` bound is added to all impls (and where clauses within
// them) except for `Default`. This is because we must access fields using
// copies (e.g. `&{self.0}`), instead of using direct references (e.g.
// `&self.0`) which may be misaligned in a packed struct.
// An empty enum.

// A single-variant enum.


// A C-like, fieldless enum with a single variant.

// A C-like, fieldless enum.

// An enum with multiple fieldless and fielded variants.

// An enum with no fieldless variants. Note that `Default` cannot be derived


// A generic enum. Note that `Default` cannot be derived for this enum.

// A union. Most builtin traits are not derivable for unions.
#![feature(internal_features_lint)]
#[prelude_import]
use std::prelude::rust_2021::*;
extern crate std;
struct Empty;
#[automatically_derived]
impl ::core::clone::Clone for Empty {
impl ::core::clone::Clone for Empty {
    #[inline]
    fn clone(&self) -> Empty { *self }
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for Empty { }
#[automatically_derived]
impl ::core::fmt::Debug for Empty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Empty")
}
#[automatically_derived]
impl ::core::default::Default for Empty {
    #[inline]
    #[inline]
    fn default() -> Empty { Empty {} }
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for Empty {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Empty { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Empty {
    #[inline]
    fn eq(&self, other: &Empty) -> bool { true }
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Empty { }
#[automatically_derived]
impl ::core::cmp::Eq for Empty {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Empty {
    #[inline]
    fn partial_cmp(&self, other: &Empty)
        -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for Empty {
    #[inline]
    fn cmp(&self, other: &Empty) -> ::core::cmp::Ordering {
        ::core::cmp::Ordering::Equal
}
struct Point {
    x: u32,
    y: u32,
    y: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for Point {
    #[inline]
    fn clone(&self) -> Point {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
}
#[automatically_derived]
impl ::core::marker::Copy for Point { }
#[automatically_derived]
#[automatically_derived]
impl ::core::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(f, "Point", "x",
            &self.x, "y", &&self.y)
}
#[automatically_derived]
impl ::core::default::Default for Point {
    #[inline]
    #[inline]
    fn default() -> Point {
        Point {
            x: ::core::default::Default::default(),
            y: ::core::default::Default::default(),
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Point {
impl ::core::hash::Hash for Point {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.x, state);
        ::core::hash::Hash::hash(&self.y, state)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Point { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Point {
    #[inline]
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Point { }
#[automatically_derived]
impl ::core::cmp::Eq for Point {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Point {
    #[inline]
    #[inline]
    fn partial_cmp(&self, other: &Point)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.x, &other.x) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                ::core::cmp::PartialOrd::partial_cmp(&self.y, &other.y),
            cmp => cmp,
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Point {
impl ::core::cmp::Ord for Point {
    #[inline]
    fn cmp(&self, other: &Point) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.x, &other.x) {
            ::core::cmp::Ordering::Equal =>
                ::core::cmp::Ord::cmp(&self.y, &other.y),
            cmp => cmp,
    }
}
}
#[repr(packed)]
struct PackedPoint {
    y: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for PackedPoint {
impl ::core::clone::Clone for PackedPoint {
    #[inline]
    fn clone(&self) -> PackedPoint {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for PackedPoint { }
#[automatically_derived]
impl ::core::fmt::Debug for PackedPoint {
---
    b6: u32,
    b7: u32,
    b8: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for Big {
    fn clone(&self) -> Big {
    fn clone(&self) -> Big {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for Big { }
#[automatically_derived]
impl ::core::fmt::Debug for Big {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ =
            &["b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"];
        let values: &[&dyn ::core::fmt::Debug] =
            &[&self.b1, &self.b2, &self.b3, &self.b4, &self.b5, &self.b6,
                        &self.b7, &&self.b8];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "Big", names,
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::default::Default for Big {
    fn default() -> Big {
        Big {
        Big {
            b1: ::core::default::Default::default(),
            b2: ::core::default::Default::default(),
            b3: ::core::default::Default::default(),
            b4: ::core::default::Default::default(),
            b5: ::core::default::Default::default(),
            b6: ::core::default::Default::default(),
            b7: ::core::default::Default::default(),
            b8: ::core::default::Default::default(),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for Big {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.b1, state);
        ::core::hash::Hash::hash(&self.b2, state);
        ::core::hash::Hash::hash(&self.b3, state);
        ::core::hash::Hash::hash(&self.b4, state);
        ::core::hash::Hash::hash(&self.b5, state);
        ::core::hash::Hash::hash(&self.b6, state);
        ::core::hash::Hash::hash(&self.b7, state);
        ::core::hash::Hash::hash(&self.b8, state)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Big { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Big {
    #[inline]
    fn eq(&self, other: &Big) -> bool {
        self.b1 == other.b1 && self.b2 == other.b2 && self.b3 == other.b3 &&
                            self.b4 == other.b4 && self.b5 == other.b5 &&
                    self.b6 == other.b6 && self.b7 == other.b7 &&
            self.b8 == other.b8
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Big { }
#[automatically_derived]
impl ::core::cmp::Eq for Big {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Big {
    #[inline]
    fn partial_cmp(&self, other: &Big)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.b1, &other.b1) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                match ::core::cmp::PartialOrd::partial_cmp(&self.b2,
                        &other.b2) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        =>
                        match ::core::cmp::PartialOrd::partial_cmp(&self.b3,
                                &other.b3) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                =>
                                match ::core::cmp::PartialOrd::partial_cmp(&self.b4,
                                        &other.b4) {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                        =>
                                        match ::core::cmp::PartialOrd::partial_cmp(&self.b5,
                                                &other.b5) {
                                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                                =>
                                                match ::core::cmp::PartialOrd::partial_cmp(&self.b6,
                                                        &other.b6) {
                                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                                        =>
                                                        match ::core::cmp::PartialOrd::partial_cmp(&self.b7,
                                                                &other.b7) {
                                                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                                                =>
                                                                ::core::cmp::PartialOrd::partial_cmp(&self.b8, &other.b8),
                                                            cmp => cmp,
                                                    cmp => cmp,
                                                },
                                            cmp => cmp,
                                        },
---
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Big {
    #[inline]
    fn cmp(&self, other: &Big) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.b1, &other.b1) {
            ::core::cmp::Ordering::Equal =>
                match ::core::cmp::Ord::cmp(&self.b2, &other.b2) {
                    ::core::cmp::Ordering::Equal =>
                        match ::core::cmp::Ord::cmp(&self.b3, &other.b3) {
                            ::core::cmp::Ordering::Equal =>
                                match ::core::cmp::Ord::cmp(&self.b4, &other.b4) {
                                    ::core::cmp::Ordering::Equal =>
                                        match ::core::cmp::Ord::cmp(&self.b5, &other.b5) {
                                            ::core::cmp::Ordering::Equal =>
                                                match ::core::cmp::Ord::cmp(&self.b6, &other.b6) {
                                                    ::core::cmp::Ordering::Equal =>
                                                        match ::core::cmp::Ord::cmp(&self.b7, &other.b7) {
                                                            ::core::cmp::Ordering::Equal =>
                                                                ::core::cmp::Ord::cmp(&self.b8, &other.b8),
                                                            cmp => cmp,
                                                    cmp => cmp,
                                                },
                                            cmp => cmp,
                                        },
---
            cmp => cmp,
        }
    }
}
struct NonCopy(u32);
#[automatically_derived]
impl ::core::clone::Clone for NonCopy {
    fn clone(&self) -> NonCopy {
    fn clone(&self) -> NonCopy {
        NonCopy(::core::clone::Clone::clone(&self.0))
}
}
#[repr(packed)]
struct PackedNonCopy(u32);
#[automatically_derived]
impl ::core::clone::Clone for PackedNonCopy {
    #[inline]
    fn clone(&self) -> PackedNonCopy {
        PackedNonCopy(::core::clone::Clone::clone(&{ self.0 }))
}
struct ManualCopy(u32);
#[automatically_derived]
impl ::core::clone::Clone for ManualCopy {
impl ::core::clone::Clone for ManualCopy {
    #[inline]
    fn clone(&self) -> ManualCopy {
        ManualCopy(::core::clone::Clone::clone(&self.0))
}
impl Copy for ManualCopy {}
impl Copy for ManualCopy {}
#[repr(packed)]
struct PackedManualCopy(u32);
#[automatically_derived]
impl ::core::clone::Clone for PackedManualCopy {
    #[inline]
    fn clone(&self) -> PackedManualCopy {
        PackedManualCopy(::core::clone::Clone::clone(&{ self.0 }))
}
}
impl Copy for PackedManualCopy {}
struct Unsized([u32]);
#[automatically_derived]
impl ::core::fmt::Debug for Unsized {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Unsized",
            &&self.0)
}
#[automatically_derived]
impl ::core::hash::Hash for Unsized {
impl ::core::hash::Hash for Unsized {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Unsized { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Unsized {
    #[inline]
    fn eq(&self, other: &Unsized) -> bool { self.0 == other.0 }
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Unsized { }
#[automatically_derived]
impl ::core::cmp::Eq for Unsized {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u32]>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Unsized {
    #[inline]
    fn partial_cmp(&self, other: &Unsized)
        -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for Unsized {
    #[inline]
    fn cmp(&self, other: &Unsized) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
}
}
#[repr(packed)]
struct PackedUnsizedU8([u8]);
#[automatically_derived]
impl ::core::fmt::Debug for PackedUnsizedU8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f,
            "PackedUnsizedU8", &&self.0)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for PackedUnsizedU8 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
}
trait Trait {
    type A;
}
}
struct Generic<T: Trait, U> {
    t: T,
    ta: T::A,
    u: U,
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::clone::Clone + Trait, U: ::core::clone::Clone>
    ::core::clone::Clone for Generic<T, U> where T::A: ::core::clone::Clone {
    #[inline]
    fn clone(&self) -> Generic<T, U> {
        Generic {
            t: ::core::clone::Clone::clone(&self.t),
            ta: ::core::clone::Clone::clone(&self.ta),
            u: ::core::clone::Clone::clone(&self.u),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::marker::Copy + Trait, U: ::core::marker::Copy>
    ::core::marker::Copy for Generic<T, U> where T::A: ::core::marker::Copy {
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::fmt::Debug + Trait, U: ::core::fmt::Debug> ::core::fmt::Debug
    for Generic<T, U> where T::A: ::core::fmt::Debug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(f, "Generic", "t",
            &self.t, "ta", &self.ta, "u", &&self.u)
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::default::Default + Trait, U: ::core::default::Default>
    ::core::default::Default for Generic<T, U> where
    T::A: ::core::default::Default {
    #[inline]
    fn default() -> Generic<T, U> {
        Generic {
            t: ::core::default::Default::default(),
            ta: ::core::default::Default::default(),
            u: ::core::default::Default::default(),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::hash::Hash + Trait, U: ::core::hash::Hash> ::core::hash::Hash
    for Generic<T, U> where T::A: ::core::hash::Hash {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.t, state);
        ::core::hash::Hash::hash(&self.ta, state);
        ::core::hash::Hash::hash(&self.u, state)
}
#[automatically_derived]
#[automatically_derived]
impl<T: Trait, U> ::core::marker::StructuralPartialEq for Generic<T, U> { }
#[automatically_derived]
impl<T: ::core::cmp::PartialEq + Trait, U: ::core::cmp::PartialEq>
    ::core::cmp::PartialEq for Generic<T, U> where
    T::A: ::core::cmp::PartialEq {
    #[inline]
    fn eq(&self, other: &Generic<T, U>) -> bool {
        self.t == other.t && self.ta == other.ta && self.u == other.u
}
#[automatically_derived]
#[automatically_derived]
impl<T: Trait, U> ::core::marker::StructuralEq for Generic<T, U> { }
#[automatically_derived]
impl<T: ::core::cmp::Eq + Trait, U: ::core::cmp::Eq> ::core::cmp::Eq for
    Generic<T, U> where T::A: ::core::cmp::Eq {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T>;
        let _: ::core::cmp::AssertParamIsEq<T::A>;
        let _: ::core::cmp::AssertParamIsEq<U>;
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::cmp::PartialOrd + Trait, U: ::core::cmp::PartialOrd>
    ::core::cmp::PartialOrd for Generic<T, U> where
    T::A: ::core::cmp::PartialOrd {
    #[inline]
    fn partial_cmp(&self, other: &Generic<T, U>)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.t, &other.t) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                match ::core::cmp::PartialOrd::partial_cmp(&self.ta,
                        &other.ta) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        => ::core::cmp::PartialOrd::partial_cmp(&self.u, &other.u),
                    cmp => cmp,
            cmp => cmp,
        }
    }
}
}
#[automatically_derived]
impl<T: ::core::cmp::Ord + Trait, U: ::core::cmp::Ord> ::core::cmp::Ord for
    Generic<T, U> where T::A: ::core::cmp::Ord {
    #[inline]
    fn cmp(&self, other: &Generic<T, U>) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.t, &other.t) {
            ::core::cmp::Ordering::Equal =>
                match ::core::cmp::Ord::cmp(&self.ta, &other.ta) {
                    ::core::cmp::Ordering::Equal =>
                        ::core::cmp::Ord::cmp(&self.u, &other.u),
                    cmp => cmp,
            cmp => cmp,
        }
    }
}
}
#[repr(packed)]
struct PackedGeneric<T: Trait, U>(T, T::A, U);
#[automatically_derived]
impl<T: ::core::clone::Clone + ::core::marker::Copy + Trait,
    U: ::core::clone::Clone + ::core::marker::Copy> ::core::clone::Clone for
    PackedGeneric<T, U> where T::A: ::core::clone::Clone +
    ::core::marker::Copy {
    #[inline]
    fn clone(&self) -> PackedGeneric<T, U> {
        PackedGeneric(::core::clone::Clone::clone(&{ self.0 }),
            ::core::clone::Clone::clone(&{ self.1 }),
            ::core::clone::Clone::clone(&{ self.2 }))
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::marker::Copy + Trait, U: ::core::marker::Copy>
    ::core::marker::Copy for PackedGeneric<T, U> where
    T::A: ::core::marker::Copy {
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::fmt::Debug + ::core::marker::Copy + Trait,
    U: ::core::fmt::Debug + ::core::marker::Copy> ::core::fmt::Debug for
    PackedGeneric<T, U> where T::A: ::core::fmt::Debug + ::core::marker::Copy
    {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field3_finish(f, "PackedGeneric",
            &{ self.0 }, &{ self.1 }, &&{ self.2 })
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::default::Default + Trait, U: ::core::default::Default>
    ::core::default::Default for PackedGeneric<T, U> where
    T::A: ::core::default::Default {
    #[inline]
    fn default() -> PackedGeneric<T, U> {
        PackedGeneric(::core::default::Default::default(),
            ::core::default::Default::default(),
            ::core::default::Default::default())
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::hash::Hash + ::core::marker::Copy + Trait,
    U: ::core::hash::Hash + ::core::marker::Copy> ::core::hash::Hash for
    PackedGeneric<T, U> where T::A: ::core::hash::Hash + ::core::marker::Copy
    {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&{ self.0 }, state);
        ::core::hash::Hash::hash(&{ self.1 }, state);
        ::core::hash::Hash::hash(&{ self.2 }, state)
}
#[automatically_derived]
#[automatically_derived]
impl<T: Trait, U> ::core::marker::StructuralPartialEq for PackedGeneric<T, U>
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::cmp::PartialEq + ::core::marker::Copy + Trait,
    U: ::core::cmp::PartialEq + ::core::marker::Copy> ::core::cmp::PartialEq
    for PackedGeneric<T, U> where T::A: ::core::cmp::PartialEq +
    ::core::marker::Copy {
    #[inline]
    fn eq(&self, other: &PackedGeneric<T, U>) -> bool {
        ({ self.0 }) == ({ other.0 }) && ({ self.1 }) == ({ other.1 }) &&
            ({ self.2 }) == ({ other.2 })
}
#[automatically_derived]
#[automatically_derived]
impl<T: Trait, U> ::core::marker::StructuralEq for PackedGeneric<T, U> { }
#[automatically_derived]
impl<T: ::core::cmp::Eq + ::core::marker::Copy + Trait, U: ::core::cmp::Eq +
    ::core::marker::Copy> ::core::cmp::Eq for PackedGeneric<T, U> where
    T::A: ::core::cmp::Eq + ::core::marker::Copy {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T>;
        let _: ::core::cmp::AssertParamIsEq<T::A>;
        let _: ::core::cmp::AssertParamIsEq<U>;
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::cmp::PartialOrd + ::core::marker::Copy + Trait,
    U: ::core::cmp::PartialOrd + ::core::marker::Copy> ::core::cmp::PartialOrd
    for PackedGeneric<T, U> where T::A: ::core::cmp::PartialOrd +
    ::core::marker::Copy {
    #[inline]
    fn partial_cmp(&self, other: &PackedGeneric<T, U>)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&{ self.0 }, &{ other.0 })
            {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                match ::core::cmp::PartialOrd::partial_cmp(&{ self.1 },
                        &{ other.1 }) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        =>
                        ::core::cmp::PartialOrd::partial_cmp(&{ self.2 },
                            &{ other.2 }),
                    cmp => cmp,
            cmp => cmp,
        }
    }
}
}
#[automatically_derived]
impl<T: ::core::cmp::Ord + ::core::marker::Copy + Trait, U: ::core::cmp::Ord +
    ::core::marker::Copy> ::core::cmp::Ord for PackedGeneric<T, U> where
    T::A: ::core::cmp::Ord + ::core::marker::Copy {
    #[inline]
    fn cmp(&self, other: &PackedGeneric<T, U>) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&{ self.0 }, &{ other.0 }) {
            ::core::cmp::Ordering::Equal =>
                match ::core::cmp::Ord::cmp(&{ self.1 }, &{ other.1 }) {
                    ::core::cmp::Ordering::Equal =>
                        ::core::cmp::Ord::cmp(&{ self.2 }, &{ other.2 }),
                    cmp => cmp,
            cmp => cmp,
        }
    }
}
}
enum Enum0 {}
#[automatically_derived]
impl ::core::clone::Clone for Enum0 {
    #[inline]
    fn clone(&self) -> Enum0 { *self }
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for Enum0 { }
#[automatically_derived]
impl ::core::fmt::Debug for Enum0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        unsafe { ::core::intrinsics::unreachable() }
}
#[automatically_derived]
impl ::core::hash::Hash for Enum0 {
impl ::core::hash::Hash for Enum0 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        unsafe { ::core::intrinsics::unreachable() }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Enum0 { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Enum0 {
    #[inline]
    fn eq(&self, other: &Enum0) -> bool {
        unsafe { ::core::intrinsics::unreachable() }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Enum0 { }
#[automatically_derived]
impl ::core::cmp::Eq for Enum0 {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Enum0 {
    #[inline]
    #[inline]
    fn partial_cmp(&self, other: &Enum0)
        -> ::core::option::Option<::core::cmp::Ordering> {
        unsafe { ::core::intrinsics::unreachable() }
}
#[automatically_derived]
impl ::core::cmp::Ord for Enum0 {
    #[inline]
    #[inline]
    fn cmp(&self, other: &Enum0) -> ::core::cmp::Ordering {
        unsafe { ::core::intrinsics::unreachable() }
}
enum Enum1 {
    Single {
        x: u32,
        x: u32,
    },
}
#[automatically_derived]
impl ::core::clone::Clone for Enum1 {
    #[inline]
    fn clone(&self) -> Enum1 {
        match self {
            Enum1::Single { x: __self_0 } =>
                Enum1::Single { x: ::core::clone::Clone::clone(__self_0) },
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Enum1 {
impl ::core::fmt::Debug for Enum1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Enum1::Single { x: __self_0 } =>
                ::core::fmt::Formatter::debug_struct_field1_finish(f,
                    "Single", "x", &__self_0),
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Enum1 {
impl ::core::hash::Hash for Enum1 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match self {
            Enum1::Single { x: __self_0 } =>
                ::core::hash::Hash::hash(__self_0, state),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Enum1 { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Enum1 {
    #[inline]
    fn eq(&self, other: &Enum1) -> bool {
        match (self, other) {
            (Enum1::Single { x: __self_0 }, Enum1::Single { x: __arg1_0 }) =>
                *__self_0 == *__arg1_0,
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Enum1 { }
#[automatically_derived]
impl ::core::cmp::Eq for Enum1 {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Enum1 {
    #[inline]
    #[inline]
    fn partial_cmp(&self, other: &Enum1)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match (self, other) {
            (Enum1::Single { x: __self_0 }, Enum1::Single { x: __arg1_0 }) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Enum1 {
impl ::core::cmp::Ord for Enum1 {
    #[inline]
    fn cmp(&self, other: &Enum1) -> ::core::cmp::Ordering {
        match (self, other) {
            (Enum1::Single { x: __self_0 }, Enum1::Single { x: __arg1_0 }) =>
                ::core::cmp::Ord::cmp(__self_0, __arg1_0),
    }
}
enum Fieldless1 {


    #[default]
    A,
}
#[automatically_derived]
impl ::core::clone::Clone for Fieldless1 {
    #[inline]
    fn clone(&self) -> Fieldless1 { Fieldless1::A }
#[automatically_derived]
#[automatically_derived]
impl ::core::fmt::Debug for Fieldless1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "A")
}
#[automatically_derived]
impl ::core::default::Default for Fieldless1 {
    #[inline]
    #[inline]
    fn default() -> Fieldless1 { Self::A }
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for Fieldless1 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Fieldless1 { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Fieldless1 {
    #[inline]
    fn eq(&self, other: &Fieldless1) -> bool { true }
#[automatically_derived]
---
}
#[automatically_derived]
impl ::core::clone::Clone for Mixed {
    #[inline]
    fn clone(&self) -> Mixed {
        let _: ::core::clone::AssertParamIsClone<u32>;
        let _: ::core::clone::AssertParamIsClone<Option<u32>>;
        let _: ::core::clone::AssertParamIsClone<Option<i32>>;
        *self
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for Mixed { }
#[automatically_derived]
impl ::core::fmt::Debug for Mixed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Mixed::P => ::core::fmt::Formatter::write_str(f, "P"),
            Mixed::Q => ::core::fmt::Formatter::write_str(f, "Q"),
            Mixed::R(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "R",
                    &__self_0),
            Mixed::S { d1: __self_0, d2: __self_1 } =>
                ::core::fmt::Formatter::debug_struct_field2_finish(f, "S",
                    "d1", __self_0, "d2", &__self_1),
    }
}
#[automatically_derived]
impl ::core::default::Default for Mixed {
impl ::core::default::Default for Mixed {
    #[inline]
    fn default() -> Mixed { Self::P }
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for Mixed {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state);
        match self {
            Mixed::R(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            Mixed::S { d1: __self_0, d2: __self_1 } => {
                ::core::hash::Hash::hash(__self_0, state);
                ::core::hash::Hash::hash(__self_1, state)
            _ => {}
        }
    }
}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Mixed { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Mixed {
    #[inline]
    fn eq(&self, other: &Mixed) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag &&
            match (self, other) {
                (Mixed::R(__self_0), Mixed::R(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                (Mixed::S { d1: __self_0, d2: __self_1 }, Mixed::S {
                    d1: __arg1_0, d2: __arg1_1 }) =>
                    *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
            }
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Mixed { }
#[automatically_derived]
impl ::core::cmp::Eq for Mixed {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<Option<u32>>;
        let _: ::core::cmp::AssertParamIsEq<Option<i32>>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Mixed {
    #[inline]
    fn partial_cmp(&self, other: &Mixed)
        -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match (self, other) {
            (Mixed::R(__self_0), Mixed::R(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            (Mixed::S { d1: __self_0, d2: __self_1 }, Mixed::S {
                d1: __arg1_0, d2: __arg1_1 }) =>
                match ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        => ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1),
                    cmp => cmp,
            _ =>
            _ =>
                ::core::cmp::PartialOrd::partial_cmp(&__self_tag,
                    &__arg1_tag),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for Mixed {
    #[inline]
    fn cmp(&self, other: &Mixed) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
            ::core::cmp::Ordering::Equal =>
                match (self, other) {
                    (Mixed::R(__self_0), Mixed::R(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (Mixed::S { d1: __self_0, d2: __self_1 }, Mixed::S {
                        d1: __arg1_0, d2: __arg1_1 }) =>
                        match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                            ::core::cmp::Ordering::Equal =>
                                ::core::cmp::Ord::cmp(__self_1, __arg1_1),
                            cmp => cmp,
                        },
                    _ => ::core::cmp::Ordering::Equal,
            cmp => cmp,
        }
    }
}
}
enum Fielded { X(u32), Y(bool), Z(Option<i32>), }
#[automatically_derived]
impl ::core::clone::Clone for Fielded {
    fn clone(&self) -> Fielded {
        match self {
        match self {
            Fielded::X(__self_0) =>
                Fielded::X(::core::clone::Clone::clone(__self_0)),
            Fielded::Y(__self_0) =>
                Fielded::Y(::core::clone::Clone::clone(__self_0)),
            Fielded::Z(__self_0) =>
                Fielded::Z(::core::clone::Clone::clone(__self_0)),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::fmt::Debug for Fielded {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Fielded::X(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X",
                    &__self_0),
            Fielded::Y(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Y",
                    &__self_0),
            Fielded::Z(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Z",
                    &__self_0),
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Fielded {
impl ::core::hash::Hash for Fielded {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state);
        match self {
            Fielded::X(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            Fielded::Y(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            Fielded::Z(__self_0) => ::core::hash::Hash::hash(__self_0, state),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Fielded { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Fielded {
    #[inline]
    fn eq(&self, other: &Fielded) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag &&
            match (self, other) {
                (Fielded::X(__self_0), Fielded::X(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                (Fielded::Y(__self_0), Fielded::Y(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                (Fielded::Z(__self_0), Fielded::Z(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Fielded { }
#[automatically_derived]
impl ::core::cmp::Eq for Fielded {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<bool>;
        let _: ::core::cmp::AssertParamIsEq<Option<i32>>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Fielded {
    #[inline]
    fn partial_cmp(&self, other: &Fielded)
        -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match (self, other) {
            (Fielded::X(__self_0), Fielded::X(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            (Fielded::Y(__self_0), Fielded::Y(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            (Fielded::Z(__self_0), Fielded::Z(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            _ =>
                ::core::cmp::PartialOrd::partial_cmp(&__self_tag,
                    &__arg1_tag),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for Fielded {
    #[inline]
    fn cmp(&self, other: &Fielded) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
            ::core::cmp::Ordering::Equal =>
                match (self, other) {
                    (Fielded::X(__self_0), Fielded::X(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (Fielded::Y(__self_0), Fielded::Y(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (Fielded::Z(__self_0), Fielded::Z(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() }
            cmp => cmp,
        }
    }
}
}
enum EnumGeneric<T, U> { One(T), Two(U), }
#[automatically_derived]
impl<T: ::core::clone::Clone, U: ::core::clone::Clone> ::core::clone::Clone
    for EnumGeneric<T, U> {
    #[inline]
    fn clone(&self) -> EnumGeneric<T, U> {
        match self {
            EnumGeneric::One(__self_0) =>
                EnumGeneric::One(::core::clone::Clone::clone(__self_0)),
            EnumGeneric::Two(__self_0) =>
                EnumGeneric::Two(::core::clone::Clone::clone(__self_0)),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::marker::Copy, U: ::core::marker::Copy> ::core::marker::Copy
    for EnumGeneric<T, U> {
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::fmt::Debug, U: ::core::fmt::Debug> ::core::fmt::Debug for
    EnumGeneric<T, U> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            EnumGeneric::One(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "One",
                    &__self_0),
            EnumGeneric::Two(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Two",
                    &__self_0),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::hash::Hash, U: ::core::hash::Hash> ::core::hash::Hash for
    EnumGeneric<T, U> {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state);
        match self {
            EnumGeneric::One(__self_0) =>
                ::core::hash::Hash::hash(__self_0, state),
            EnumGeneric::Two(__self_0) =>
                ::core::hash::Hash::hash(__self_0, state),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T, U> ::core::marker::StructuralPartialEq for EnumGeneric<T, U> { }
#[automatically_derived]
impl<T: ::core::cmp::PartialEq, U: ::core::cmp::PartialEq>
    ::core::cmp::PartialEq for EnumGeneric<T, U> {
    #[inline]
    fn eq(&self, other: &EnumGeneric<T, U>) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag &&
            match (self, other) {
                (EnumGeneric::One(__self_0), EnumGeneric::One(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                (EnumGeneric::Two(__self_0), EnumGeneric::Two(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T, U> ::core::marker::StructuralEq for EnumGeneric<T, U> { }
#[automatically_derived]
impl<T: ::core::cmp::Eq, U: ::core::cmp::Eq> ::core::cmp::Eq for
    EnumGeneric<T, U> {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T>;
        let _: ::core::cmp::AssertParamIsEq<U>;
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::cmp::PartialOrd, U: ::core::cmp::PartialOrd>
    ::core::cmp::PartialOrd for EnumGeneric<T, U> {
    #[inline]
    fn partial_cmp(&self, other: &EnumGeneric<T, U>)
        -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match (self, other) {
            (EnumGeneric::One(__self_0), EnumGeneric::One(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            (EnumGeneric::Two(__self_0), EnumGeneric::Two(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            _ =>
                ::core::cmp::PartialOrd::partial_cmp(&__self_tag,
                    &__arg1_tag),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::cmp::Ord, U: ::core::cmp::Ord> ::core::cmp::Ord for
    EnumGeneric<T, U> {
    #[inline]
    fn cmp(&self, other: &EnumGeneric<T, U>) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
            ::core::cmp::Ordering::Equal =>
                match (self, other) {
                    (EnumGeneric::One(__self_0), EnumGeneric::One(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (EnumGeneric::Two(__self_0), EnumGeneric::Two(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() }
            cmp => cmp,
        }
    }
}
}
pub union Union {
    pub b: bool,
    pub u: u32,
    pub i: i32,
#[automatically_derived]
impl ::core::clone::Clone for Union {
    #[inline]
    fn clone(&self) -> Union {
    fn clone(&self) -> Union {
        let _: ::core::clone::AssertParamIsCopy<Self>;
        *self
}
#[automatically_derived]
impl ::core::marker::Copy for Union { }
------------------------------------------
------------------------------------------
--- stderr -------------------------------
warning: byte slice in a packed struct that derives a built-in trait
  --> fake-test-src-base/deriving/deriving-all-codegen.rs:80:24
   |
LL | #[derive(Debug, Hash)]
   |          ----- in this derive macro expansion
LL | #[repr(packed)]
LL | struct PackedUnsizedU8([u8]);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = help: consider implementing the trait by hand, or remove the `packed` attribute
   = note: `#[warn(byte_slice_in_packed_struct_with_derive)]` on by default

warning: byte slice in a packed struct that derives a built-in trait
  --> fake-test-src-base/deriving/deriving-all-codegen.rs:80:24
   |
   |
LL | #[derive(Debug, Hash)]
   |                 ---- in this derive macro expansion
LL | #[repr(packed)]
LL | struct PackedUnsizedU8([u8]);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = help: consider implementing the trait by hand, or remove the `packed` attribute
   = note: this warning originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 2 warnings emitted

Future incompatibility report: Future breakage diagnostic:
warning: byte slice in a packed struct that derives a built-in trait
warning: byte slice in a packed struct that derives a built-in trait
  --> fake-test-src-base/deriving/deriving-all-codegen.rs:80:24
   |
LL | #[derive(Debug, Hash)]
   |          ----- in this derive macro expansion
LL | #[repr(packed)]
LL | struct PackedUnsizedU8([u8]);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = help: consider implementing the trait by hand, or remove the `packed` attribute
   = note: `#[warn(byte_slice_in_packed_struct_with_derive)]` on by default

Future breakage diagnostic:
warning: byte slice in a packed struct that derives a built-in trait
  --> fake-test-src-base/deriving/deriving-all-codegen.rs:80:24
  --> fake-test-src-base/deriving/deriving-all-codegen.rs:80:24
   |
LL | #[derive(Debug, Hash)]
   |                 ---- in this derive macro expansion
LL | #[repr(packed)]
LL | struct PackedUnsizedU8([u8]);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = help: consider implementing the trait by hand, or remove the `packed` attribute
   = note: `#[warn(byte_slice_in_packed_struct_with_derive)]` on by default
   = note: this warning originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)


---- [ui] tests/ui/feature-gates/allow-features-empty.rs stdout ----
diff of stderr:
---
To only update this specific test, also pass `--test-args feature-gates/allow-features-empty.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/feature-gates/allow-features-empty.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/allow-features-empty" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/allow-features-empty/auxiliary" "-Z" "allow_features="
stdout: none
--- stderr -------------------------------
error[E0725]: the feature `lang_items` is not in the list of allowed features
  --> fake-test-src-base/feature-gates/allow-features-empty.rs:4:12
   |
LL | #![feature(lang_items)] //~ ERROR

error[E0725]: the feature `unknown_stdlib_feature` is not in the list of allowed features
  --> fake-test-src-base/feature-gates/allow-features-empty.rs:6:12
   |
   |
LL | #![feature(unknown_stdlib_feature)] //~ ERROR

error[E0725]: the feature `internal_features_lint` is not in the list of allowed features
  --> <crate attribute>:1:9
   |
---
+    |
+    = note: the `internal_features` lint is unstable
+    = help: add `#![feature(internal_features_lint)]` to the crate attributes to enable
+ 
+ error: the feature `lang_items` is internal to the compiler or standard library
+    |
+ LL | #![feature(lang_items)]
+    |            ^^^^^^^^^^
+    |
+    |
+    = note: using it is strongly discouraged
+    = note: `#[deny(internal_features)]` on by default
+ error: aborting due to 3 previous errors; 2 warnings emitted
8 
9 For more information about this error, try `rustc --explain E0725`.
10 
---
To only update this specific test, also pass `--test-args feature-gates/allow-features.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/feature-gates/allow-features.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/allow-features" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/allow-features/auxiliary" "-Z" "allow_features=lang_items"
stdout: none
error[E0725]: the feature `unknown_stdlib_feature` is not in the list of allowed features
  --> fake-test-src-base/feature-gates/allow-features.rs:6:12
   |
   |
LL | #![feature(unknown_stdlib_feature)] //~ ERROR

error[E0725]: the feature `internal_features_lint` is not in the list of allowed features
  --> <crate attribute>:1:9
   |
---
   |
   = note: the `internal_features` lint is unstable
   = help: add `#![feature(internal_features_lint)]` to the crate attributes to enable

error: the feature `lang_items` is internal to the compiler or standard library
  --> fake-test-src-base/feature-gates/allow-features.rs:4:12
LL | #![feature(lang_items)]
   |            ^^^^^^^^^^
   |
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default
error: aborting due to 3 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0725`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/feature-gates/feature-gate-feature-gate.rs stdout ----
diff of stderr:

10 LL | #![forbid(unstable_features)]
12 
- error: aborting due to previous error
+ error: unstable feature
+   --> <crate attribute>:1:9
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-feature-gate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/feature-gates/feature-gate-feature-gate.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-feature-gate" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-feature-gate/auxiliary"
stdout: none
error: unstable feature
  --> fake-test-src-base/feature-gates/feature-gate-feature-gate.rs:2:12
   |
   |
LL | #![feature(intrinsics)] //~ ERROR unstable feature
   |
note: the lint level is defined here
  --> fake-test-src-base/feature-gates/feature-gate-feature-gate.rs:1:11
   |
   |
LL | #![forbid(unstable_features)]

error: unstable feature
  --> <crate attribute>:1:9
   |
---

---- [ui] tests/ui/hygiene/unpretty-debug.rs stdout ----
diff of stdout:

8 #![feature /* 0#0 */(no_core)]
9 #![no_core /* 0#0 */]
10 
- macro_rules! foo /* 0#0 */ { ($x : ident) => { y + $x } }
+ 
+ 
+ #![feature /* 0#0 */(internal_features_lint)]
+ macro_rules! foo /* 0#0 */ { ($x : ident) => { y + $x } }
13 fn bar /* 0#0 */() {
14     let x /* 0#0 */ = 1;
15     y /* 0#1 */ + x /* 0#0 */
16 }
- 
- 
18 fn y /* 0#0 */() {}
20 /*


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/unpretty-debug.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/unpretty-debug.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/hygiene/unpretty-debug.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/auxiliary" "-Zunpretty=expanded,hygiene"
// check-pass
// check-pass
// compile-flags: -Zunpretty=expanded,hygiene

// Don't break whenever Symbol numbering changes
// normalize-stdout-test "\d+#" -> "0#"
// minimal junk
// minimal junk
#![feature /* 640#0 */(no_core)]
#![no_core /* 948#0 */]



#![feature /* 640#0 */(internal_features_lint)]
macro_rules! foo /* 1604#0 */ { ($x : ident) => { y + $x } }
fn bar /* 1607#0 */() {
    let x /* 1605#0 */ = 1;
    y /* 1606#1 */ + x /* 1605#0 */
}
fn y /* 1606#0 */() {}
/*
Expansions:
Expansions:
crate0::{{expn0}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
crate0::{{expn1}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "foo")
SyntaxContexts:
SyntaxContexts:
#0: parent: #0, outer_mark: (crate0::{{expn0}}, Opaque)
#1: parent: #0, outer_mark: (crate0::{{expn1}}, SemiTransparent)
------------------------------------------
stderr: none


---
To only update this specific test, also pass `--test-args issues/issue-34932.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-34932.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34932/a" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34932/auxiliary" "--test"
stdout: none
warning: unknown lint: `internal_features`
   |
   = note: the `internal_features` lint is unstable
   = help: add `#![feature(internal_features_lint)]` to the crate attributes to enable
---

---- [ui] tests/ui/lint/rfc-2383-lint-reason/no_ice_for_partial_compiler_runs.rs stdout ----
diff of stdout:

5 // compile-flags: -Z unpretty=expanded
7 #![feature(lint_reasons)]
+ 
+ 
+ // This `expect` will create an expectation with an unstable expectation id
+ // `while_true` is an early lint
+ #![feature(internal_features_lint)]
8 #[prelude_import]
8 #[prelude_import]
9 use ::std::prelude::rust_2015::*;

11 extern crate std;
- 
- 
- // This `expect` will create an expectation with an unstable expectation id
14 #[expect(while_true)]
- fn create_early_lint_pass_expectation() {
-     // `while_true` is an early lint
-     while true {}
- 
- 
+ fn create_early_lint_pass_expectation() { while true {} }
20 fn main() { create_early_lint_pass_expectation(); }


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/no_ice_for_partial_compiler_runs/no_ice_for_partial_compiler_runs.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/no_ice_for_partial_compiler_runs/no_ice_for_partial_compiler_runs.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/rfc-2383-lint-reason/no_ice_for_partial_compiler_runs.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/rfc-2383-lint-reason/no_ice_for_partial_compiler_runs.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/no_ice_for_partial_compiler_runs" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/no_ice_for_partial_compiler_runs/auxiliary" "-Z" "unpretty=expanded"
#![feature(prelude_import)]
#![no_std]
#![no_std]
// This ensures that ICEs like rust#94953 don't happen
// check-pass
// compile-flags: -Z unpretty=expanded
#![feature(lint_reasons)]


// This `expect` will create an expectation with an unstable expectation id
// `while_true` is an early lint
#![feature(internal_features_lint)]
#[prelude_import]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
extern crate std;
#[expect(while_true)]
fn create_early_lint_pass_expectation() { while true {} }
fn main() { create_early_lint_pass_expectation(); }
stderr: none


---- [ui] tests/ui/macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen.rs stdout ----
---- [ui] tests/ui/macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen.rs stdout ----
diff of stdout:

4 // compile-flags: -Z unpretty=expanded
6 #![feature(core_intrinsics, generic_assert, generic_assert_internals)]
+ 
+ 
+ 
+ 
+ 
+ 
+ #![feature(internal_features_lint)]
7 #[prelude_import]
8 use ::std::prelude::rust_2015::*;

10 extern crate std;
- 
- 
12 fn arbitrary_consuming_method_for_demonstration_purposes() {
13     let elem = 1i32;


20                                     (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
22                                 } as usize)) {
- 
- 
- 
- 
- 
27                 {
28                     ::std::rt::panic_fmt(format_args!("Assertion failed: elem as usize\nWith captures:\n  elem = {0:?}\n",
29                             __capture0))

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen/non-consuming-methods-have-optimized-codegen.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen/auxiliary" "-Z" "unpretty=expanded"
#![feature(prelude_import)]
#![no_std]
// check-pass
// check-pass
// compile-flags: -Z unpretty=expanded
#![feature(core_intrinsics, generic_assert, generic_assert_internals)]






#![feature(internal_features_lint)]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
extern crate std;
fn arbitrary_consuming_method_for_demonstration_purposes() {
    let elem = 1i32;
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*{
                                    (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                                } as usize)) {
                {
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem as usize\nWith captures:\n  elem = {0:?}\n",
                            __capture0))
            }
    };
}
fn addr_of() {
fn addr_of() {
    let elem = 1i32;
    {
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!&*__local_bind0) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: &elem\nWith captures:\n  elem = {0:?}\n",
                            __capture0))
            }
    };
}
fn binary() {
fn binary() {
    let elem = 1i32;
    {
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 == 1)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem == 1\nWith captures:\n  elem = {0:?}\n",
                            __capture0))
            }
    };
    {
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 >= 1)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem >= 1\nWith captures:\n  elem = {0:?}\n",
                            __capture0))
            }
    };
    {
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 > 0)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem > 0\nWith captures:\n  elem = {0:?}\n",
                            __capture0))
            }
    };
    {
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 < 3)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem < 3\nWith captures:\n  elem = {0:?}\n",
                            __capture0))
            }
    };
    {
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 <= 3)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem <= 3\nWith captures:\n  elem = {0:?}\n",
                            __capture0))
            }
    };
    {
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 != 3)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem != 3\nWith captures:\n  elem = {0:?}\n",
                            __capture0))
            }
    };
}
fn unary() {
fn unary() {
    let elem = &1i32;
    {
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!**__local_bind0) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: *elem\nWith captures:\n  elem = {0:?}\n",
                            __capture0))
            }
    };
}
fn main() {}
fn main() {}
------------------------------------------
stderr: none


---- [ui] tests/ui/match/issue-82392.rs stdout ----
diff of stdout:

- #[prelude_import]
- use ::std::prelude::rust_2015::*;
- #[macro_use]
- extern crate std;
5 // https://github.com/rust-lang/rust/issues/82329
6 // compile-flags: -Zunpretty=hir,typed
7 // check-pass
8 
+ #![feature(internal_features_lint)]
+ #![feature(internal_features_lint)]
+ #[prelude_import]
+ use ::std::prelude::rust_2015::*;
+ #[macro_use]
+ extern crate std;
9 fn main() ({
10         (if (true as bool)
11                 ({ } as

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392/issue-82392.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args match/issue-82392.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/match/issue-82392.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392/auxiliary" "-Zunpretty=hir,typed"
// https://github.com/rust-lang/rust/issues/82329
// https://github.com/rust-lang/rust/issues/82329
// compile-flags: -Zunpretty=hir,typed
// check-pass
#![feature(internal_features_lint)]
#[prelude_import]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
fn main() ({
        (if (true as bool)
        (if (true as bool)
                ({ } as
                    ()) else if (let Some(a) =
                       ((Some as
                               fn(i32) -> Option<i32> {Option::<i32>::Some})((3 as i32)) as
                           Option<i32>) as bool) ({ } as ()) as ())
               } as ())
stderr: none


---- [ui] tests/ui/proc-macro/ambiguous-builtin-attrs-test.rs stdout ----
---- [ui] tests/ui/proc-macro/ambiguous-builtin-attrs-test.rs stdout ----

error: auxiliary build of "/checkout/tests/ui/proc-macro/auxiliary/builtin-attrs.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/auxiliary/builtin-attrs.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/auxiliary" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0659]: `feature` is ambiguous
  --> <crate attribute>:1:1
LL | feature(internal_features_lint)
   | ^^^^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a builtin attribute
   = note: ambiguous because of a name conflict with a builtin attribute
   = note: `feature` could refer to a built-in attribute
note: `feature` could also refer to the attribute macro defined here
  --> fake-test-src-base/proc-macro/auxiliary/builtin-attrs.rs:10:1
   |
LL | / pub fn feature(_: TokenStream, input: TokenStream) -> TokenStream {
LL | |     input
LL | | }
   | |_^
   = help: use `crate::feature` to refer to this attribute macro unambiguously
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/proc-macro/ambiguous-builtin-attrs.rs stdout ----

error: auxiliary build of "/checkout/tests/ui/proc-macro/auxiliary/builtin-attrs.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/auxiliary/builtin-attrs.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs/auxiliary" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0659]: `feature` is ambiguous
  --> <crate attribute>:1:1
LL | feature(internal_features_lint)
   | ^^^^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a builtin attribute
   = note: ambiguous because of a name conflict with a builtin attribute
   = note: `feature` could refer to a built-in attribute
note: `feature` could also refer to the attribute macro defined here
  --> fake-test-src-base/proc-macro/auxiliary/builtin-attrs.rs:10:1
   |
LL | / pub fn feature(_: TokenStream, input: TokenStream) -> TokenStream {
LL | |     input
LL | | }
   | |_^
   = help: use `crate::feature` to refer to this attribute macro unambiguously
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/proc-macro/meta-macro-hygiene.rs stdout ----
diff of stdout:

14 // in the stdout
15 
16 #![no_std /* 0#0 */]
+ // Don't load unnecessary hygiene information from std
+ 
+ 
+ // `print_def_site!` will respan the `$crate` identifier
+ // with `Span::def_site()`. This should cause it to resolve
+ // relative to `meta_macro`, *not* `make_macro` (despite
+ // the fact that `print_def_site` is produced by a
+ // `macro_rules!` macro in `make_macro`).
+ 
+ #![feature /* 0#0 */(internal_features_lint)]
17 #[prelude_import /* 0#1 */]
18 use core /* 0#1 */::prelude /* 0#1 */::rust_2018 /* 0#1 */::*;
19 #[macro_use /* 0#1 */]

20 extern crate core /* 0#1 */;
21 #[macro_use /* 0#1 */]
22 extern crate compiler_builtins /* 0#1 */;
- // Don't load unnecessary hygiene information from std
24 extern crate std /* 0#0 */;
- 
26 extern crate meta_macro /* 0#0 */;
28 macro_rules! produce_it
29     /*
30     0#0


31     */ {
-     () =>
-     {
-         meta_macro :: print_def_site! ($crate :: dummy! ()) ;
-         // `print_def_site!` will respan the `$crate` identifier
-         // with `Span::def_site()`. This should cause it to resolve
-         // relative to `meta_macro`, *not* `make_macro` (despite
-         // the fact that `print_def_site` is produced by a
-         // `macro_rules!` macro in `make_macro`).
-     }
+     () => { meta_macro :: print_def_site! ($crate :: dummy! ()) ; }
- 
- 
43 fn main /* 0#0 */() { ; }
45 /*


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene/meta-macro-hygiene.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/meta-macro-hygiene.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/meta-macro-hygiene.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene/auxiliary" "--edition=2018" "-Z" "span-debug" "-Z" "macro-backtrace" "-Z" "unpretty=expanded,hygiene" "-Z" "trim-diagnostic-paths=no"
--- stdout -------------------------------
Def site: fake-test-src-base/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5)
Input: TokenStream [Ident { ident: "$crate", span: fake-test-src-base/proc-macro/meta-macro-hygiene.rs:24:37: 24:43 (#4) }, Punct { ch: ':', spacing: Joint, span: fake-test-src-base/proc-macro/meta-macro-hygiene.rs:24:43: 24:44 (#4) }, Punct { ch: ':', spacing: Alone, span: fake-test-src-base/proc-macro/meta-macro-hygiene.rs:24:44: 24:45 (#4) }, Ident { ident: "dummy", span: fake-test-src-base/proc-macro/meta-macro-hygiene.rs:24:45: 24:50 (#4) }, Punct { ch: '!', spacing: Alone, span: fake-test-src-base/proc-macro/meta-macro-hygiene.rs:24:50: 24:51 (#4) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: fake-test-src-base/proc-macro/meta-macro-hygiene.rs:24:51: 24:53 (#4) }]
Respanned: TokenStream [Ident { ident: "$crate", span: fake-test-src-base/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Joint, span: fake-test-src-base/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Alone, span: fake-test-src-base/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Ident { ident: "dummy", span: fake-test-src-base/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: '!', spacing: Alone, span: fake-test-src-base/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: fake-test-src-base/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }]
#![feature /* 640#0 */(prelude_import)]
// aux-build:make-macro.rs
// aux-build:meta-macro.rs
// edition:2018
// compile-flags: -Z span-debug -Z macro-backtrace -Z unpretty=expanded,hygiene -Z trim-diagnostic-paths=no
// check-pass
// normalize-stdout-test "\d+#" -> "0#"
// normalize-stdout-test "expn\d{3,}" -> "expnNNN"
//
// We don't care about symbol ids, so we set them all to 0


#![no_std /* 961#0 */]
// Don't load unnecessary hygiene information from std


// `print_def_site!` will respan the `$crate` identifier
// with `Span::def_site()`. This should cause it to resolve
// relative to `meta_macro`, *not* `make_macro` (despite
// the fact that `print_def_site` is produced by a
// `macro_rules!` macro in `make_macro`).

#![feature /* 640#0 */(internal_features_lint)]
#[prelude_import /* 1058#1 */]
use core /* 488#1 */::prelude /* 1057#1 */::rust_2018 /* 1162#1 */::*;
#[macro_use /* 845#1 */]
extern crate core /* 488#1 */;
#[macro_use /* 845#1 */]
extern crate compiler_builtins /* 435#1 */;
extern crate std /* 1373#0 */;
extern crate meta_macro /* 1604#0 */;
macro_rules! produce_it
    1605#0
    */ {
    */ {
    () => { meta_macro :: print_def_site! ($crate :: dummy! ()) ; }
}
fn main /* 848#0 */() { ; }
/*
Expansions:
Expansions:
crate0::{{expn0}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
crate0::{{expn1}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
crate0::{{expn2}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "produce_it")
crate0::{{expn3}}: parent: crate0::{{expn2}}, call_site_ctxt: #4, def_site_ctxt: #0, kind: Macro(Bang, "meta_macro::print_def_site")
crate0::{{expn4}}: parent: crate0::{{expn3}}, call_site_ctxt: #5, def_site_ctxt: #0, kind: Macro(Bang, "$crate::dummy")
crate1::{{expn680}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Attr, "derive")
crate1::{{expn705}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Attr, "derive")
crate1::{{expn2768}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "include")
crate2::{{expn1}}: parent: crate0::{{expn0}}, call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
SyntaxContexts:
SyntaxContexts:
#0: parent: #0, outer_mark: (crate0::{{expn0}}, Opaque)
#1: parent: #0, outer_mark: (crate0::{{expn1}}, Opaque)
#2: parent: #0, outer_mark: (crate0::{{expn1}}, Transparent)
#3: parent: #0, outer_mark: (crate2::{{expn1}}, Opaque)
#4: parent: #0, outer_mark: (crate0::{{expn2}}, SemiTransparent)
#5: parent: #0, outer_mark: (crate0::{{expn3}}, Opaque)
#6: parent: #4, outer_mark: (crate0::{{expn3}}, Transparent)
#7: parent: #0, outer_mark: (crate0::{{expn3}}, SemiTransparent)
#8: parent: #0, outer_mark: (crate0::{{expn4}}, Opaque)
#9: parent: #5, outer_mark: (crate0::{{expn4}}, Transparent)
#10: parent: #5, outer_mark: (crate0::{{expn4}}, SemiTransparent)
------------------------------------------
stderr: none



---- [ui] tests/ui/proc-macro/quote-debug.rs stdout ----
---
To only update this specific test, also pass `--test-args stats/hir-stats.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/stats/hir-stats.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats/auxiliary" "-Zhir-stats"
stdout: none
--- stderr -------------------------------
ast-stats-1 PRE EXPANSION AST STATS
ast-stats-1 Name                Accumulated Size         Count     Item Size
ast-stats-1 ----------------------------------------------------------------
ast-stats-1 GenericArgs               40 ( 0.6%)             1            40
ast-stats-1 - AngleBracketed            40 ( 0.6%)             1
ast-stats-1 Crate                     40 ( 0.6%)             1            40
ast-stats-1 ExprField                 48 ( 0.7%)             1            48
ast-stats-1 WherePredicate            56 ( 0.9%)             1            56
ast-stats-1 - BoundPredicate            56 ( 0.9%)             1
ast-stats-1 Attribute                 64 ( 1.0%)             2            32
ast-stats-1 - Normal                    32 ( 0.5%)             1
ast-stats-1 - DocComment                32 ( 0.5%)             1
ast-stats-1 Local                     72 ( 1.1%)             1            72
ast-stats-1 Arm                       96 ( 1.5%)             2            48
ast-stats-1 ForeignItem               96 ( 1.5%)             1            96
ast-stats-1 - Fn                        96 ( 1.5%)             1
ast-stats-1 FnDecl                   120 ( 1.8%)             5            24
ast-stats-1 FieldDef                 160 ( 2.5%)             2            80
ast-stats-1 Stmt                     160 ( 2.5%)             5            32
ast-stats-1 - Local                     32 ( 0.5%)             1
ast-stats-1 - MacCall                   32 ( 0.5%)             1
ast-stats-1 - Expr                      96 ( 1.5%)             3
ast-stats-1 Param                    160 ( 2.5%)             4            40
ast-stats-1 Block                    192 ( 3.0%)             6            32
ast-stats-1 Variant                  208 ( 3.2%)             2           104
ast-stats-1 GenericBound             224 ( 3.5%)             4            56
ast-stats-1 - Trait                    224 ( 3.5%)             4
ast-stats-1 AssocItem                352 ( 5.4%)             4            88
ast-stats-1 - Type                     176 ( 2.7%)             2
ast-stats-1 - Fn                       176 ( 2.7%)             2
ast-stats-1 GenericParam             480 ( 7.4%)             5            96
ast-stats-1 Pat                      504 ( 7.8%)             7            72
ast-stats-1 - Struct                    72 ( 1.1%)             1
ast-stats-1 - Wild                      72 ( 1.1%)             1
ast-stats-1 - Ident                    360 ( 5.5%)             5
ast-stats-1 Expr                     576 ( 8.9%)             8            72
ast-stats-1 - Path                      72 ( 1.1%)             1
ast-stats-1 - Match                     72 ( 1.1%)             1
ast-stats-1 - Struct                    72 ( 1.1%)             1
ast-stats-1 - Lit                      144 ( 2.2%)             2
ast-stats-1 - Block                    216 ( 3.3%)             3
ast-stats-1 PathSegment              720 (11.1%)            30            24
ast-stats-1 Ty                       896 (13.8%)            14            64
ast-stats-1 - Ptr                       64 ( 1.0%)             1
ast-stats-1 - Ref                       64 ( 1.0%)             1
ast-stats-1 - ImplicitSelf             128 ( 2.0%)             2
ast-stats-1 - Path                     640 ( 9.9%)            10
ast-stats-1 Item                   1_224 (18.9%)             9           136
ast-stats-1 - Trait                    136 ( 2.1%)             1
ast-stats-1 - Enum                     136 ( 2.1%)             1
ast-stats-1 - ForeignMod               136 ( 2.1%)             1
ast-stats-1 - Impl                     136 ( 2.1%)             1
ast-stats-1 - Fn                       272 ( 4.2%)             2
ast-stats-1 - Use                      408 ( 6.3%)             3
ast-stats-1 ----------------------------------------------------------------
ast-stats-1 Total                  6_488
ast-stats-1
ast-stats-2 POST EXPANSION AST STATS
ast-stats-2 Name                Accumulated Size         Count     Item Size
ast-stats-2 ----------------------------------------------------------------
ast-stats-2 GenericArgs               40 ( 0.6%)             1            40
ast-stats-2 - AngleBracketed            40 ( 0.6%)             1
ast-stats-2 Crate                     40 ( 0.6%)             1            40
ast-stats-2 ExprField                 48 ( 0.7%)             1            48
ast-stats-2 WherePredicate            56 ( 0.8%)             1            56
ast-stats-2 - BoundPredicate            56 ( 0.8%)             1
ast-stats-2 Local                     72 ( 1.0%)             1            72
ast-stats-2 Arm                       96 ( 1.3%)             2            48
ast-stats-2 ForeignItem               96 ( 1.3%)             1            96
ast-stats-2 - Fn                        96 ( 1.3%)             1
ast-stats-2 InlineAsm                120 ( 1.7%)             1           120
ast-stats-2 FnDecl                   120 ( 1.7%)             5            24
ast-stats-2 Attribute                160 ( 2.2%)             5            32
ast-stats-2 - DocComment                32 ( 0.4%)             1
ast-stats-2 - Normal                   128 ( 1.8%)             4
ast-stats-2 FieldDef                 160 ( 2.2%)             2            80
ast-stats-2 Stmt                     160 ( 2.2%)             5            32
ast-stats-2 - Local                     32 ( 0.4%)             1
ast-stats-2 - Semi                      32 ( 0.4%)             1
ast-stats-2 - Expr                      96 ( 1.3%)             3
ast-stats-2 Param                    160 ( 2.2%)             4            40
ast-stats-2 Block                    192 ( 2.7%)             6            32
ast-stats-2 Variant                  208 ( 2.9%)             2           104
ast-stats-2 GenericBound             224 ( 3.1%)             4            56
ast-stats-2 - Trait                    224 ( 3.1%)             4
ast-stats-2 AssocItem                352 ( 4.9%)             4            88
ast-stats-2 - Type                     176 ( 2.5%)             2
ast-stats-2 - Fn                       176 ( 2.5%)             2
ast-stats-2 GenericParam             480 ( 6.7%)             5            96
ast-stats-2 Pat                      504 ( 7.1%)             7            72
ast-stats-2 - Struct                    72 ( 1.0%)             1
ast-stats-2 - Wild                      72 ( 1.0%)             1
ast-stats-2 - Ident                    360 ( 5.1%)             5
ast-stats-2 Expr                     648 ( 9.1%)             9            72
ast-stats-2 - Path                      72 ( 1.0%)             1
ast-stats-2 - Match                     72 ( 1.0%)             1
ast-stats-2 - Struct                    72 ( 1.0%)             1
ast-stats-2 - InlineAsm                 72 ( 1.0%)             1
ast-stats-2 - Lit                      144 ( 2.0%)             2
ast-stats-2 - Block                    216 ( 3.0%)             3
ast-stats-2 PathSegment              792 (11.1%)            33            24
ast-stats-2 Ty                       896 (12.6%)            14            64
ast-stats-2 - Ptr                       64 ( 0.9%)             1
ast-stats-2 - Ref                       64 ( 0.9%)             1
ast-stats-2 - ImplicitSelf             128 ( 1.8%)             2
ast-stats-2 - Path                     640 ( 9.0%)            10
ast-stats-2 Item                   1_496 (21.0%)            11           136
ast-stats-2 - Trait                    136 ( 1.9%)             1
ast-stats-2 - Enum                     136 ( 1.9%)             1
ast-stats-2 - ExternCrate              136 ( 1.9%)             1
ast-stats-2 - ForeignMod               136 ( 1.9%)             1
ast-stats-2 - Impl                     136 ( 1.9%)             1
ast-stats-2 - Fn                       272 ( 3.8%)             2
ast-stats-2 - Use                      544 ( 7.6%)             4
ast-stats-2 ----------------------------------------------------------------
ast-stats-2 Total                  7_120
hir-stats HIR STATS
hir-stats HIR STATS
hir-stats Name                Accumulated Size         Count     Item Size
hir-stats ----------------------------------------------------------------
hir-stats ForeignItemRef            24 ( 0.3%)             1            24
hir-stats Lifetime                  24 ( 0.3%)             1            24
hir-stats Mod                       32 ( 0.4%)             1            32
hir-stats ExprField                 40 ( 0.4%)             1            40
hir-stats TraitItemRef              56 ( 0.6%)             2            28
hir-stats Local                     64 ( 0.7%)             1            64
hir-stats Param                     64 ( 0.7%)             2            32
hir-stats InlineAsm                 72 ( 0.8%)             1            72
hir-stats ImplItemRef               72 ( 0.8%)             2            36
hir-stats Body                      96 ( 1.1%)             3            32
hir-stats FieldDef                  96 ( 1.1%)             2            48
hir-stats Arm                       96 ( 1.1%)             2            48
hir-stats Stmt                      96 ( 1.1%)             3            32
hir-stats - Local                     32 ( 0.4%)             1
hir-stats - Semi                      32 ( 0.4%)             1
hir-stats - Expr                      32 ( 0.4%)             1
hir-stats FnDecl                   120 ( 1.3%)             3            40
hir-stats GenericArg               128 ( 1.4%)             4            32
hir-stats - Type                      32 ( 0.4%)             1
hir-stats - Lifetime                  96 ( 1.1%)             3
hir-stats GenericArgs              144 ( 1.6%)             3            48
hir-stats Attribute                160 ( 1.8%)             5            32
hir-stats Variant                  176 ( 1.9%)             2            88
hir-stats GenericBound             192 ( 2.1%)             4            48
hir-stats - Trait                    192 ( 2.1%)             4
hir-stats WherePredicate           192 ( 2.1%)             3            64
hir-stats - BoundPredicate           192 ( 2.1%)             3
hir-stats Block                    288 ( 3.2%)             6            48
hir-stats Pat                      360 ( 4.0%)             5            72
hir-stats - Wild                      72 ( 0.8%)             1
hir-stats - Struct                    72 ( 0.8%)             1
hir-stats - Binding                  216 ( 2.4%)             3
hir-stats GenericParam             400 ( 4.4%)             5            80
hir-stats Generics                 560 ( 6.2%)            10            56
hir-stats Ty                       720 ( 7.9%)            15            48
hir-stats - Ptr                       48 ( 0.5%)             1
hir-stats - Ref                       48 ( 0.5%)             1
hir-stats - Path                     624 ( 6.9%)            13
hir-stats Expr                     768 ( 8.5%)            12            64
hir-stats - Path                      64 ( 0.7%)             1
hir-stats - Struct                    64 ( 0.7%)             1
hir-stats - Match                     64 ( 0.7%)             1
hir-stats - InlineAsm                 64 ( 0.7%)             1
hir-stats - Lit                      128 ( 1.4%)             2
hir-stats - Block                    384 ( 4.2%)             6
hir-stats Item                     880 ( 9.7%)            11            80
hir-stats - Trait                     80 ( 0.9%)             1
hir-stats - Enum                      80 ( 0.9%)             1
hir-stats - ExternCrate               80 ( 0.9%)             1
hir-stats - ForeignMod                80 ( 0.9%)             1
hir-stats - Impl                      80 ( 0.9%)             1
hir-stats - Fn                       160 ( 1.8%)             2
hir-stats - Use                      320 ( 3.5%)             4
hir-stats Path                   1_240 (13.7%)            31            40
hir-stats PathSegment            1_920 (21.1%)            40            48
hir-stats ----------------------------------------------------------------
hir-stats
------------------------------------------



---- [ui] tests/ui/type-alias-impl-trait/issue-60662.rs stdout ----
diff of stdout:

2 // compile-flags: -Z unpretty=hir
4 #![feature(type_alias_impl_trait)]
+ 
+ 
+ #![feature(internal_features_lint)]
+ #![feature(internal_features_lint)]
5 #[prelude_import]
6 use ::std::prelude::rust_2015::*;

8 extern crate std;
- 
- 
10 trait Animal { }
12 fn main() {
12 fn main() {
13         type ServeFut = /*impl Trait*/;


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-60662/issue-60662.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-60662/issue-60662.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-60662.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/issue-60662.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-60662" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-60662/auxiliary" "-Z" "unpretty=hir"
// check-pass
// check-pass
// compile-flags: -Z unpretty=hir
#![feature(type_alias_impl_trait)]


#![feature(internal_features_lint)]
#![feature(internal_features_lint)]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
extern crate std;
trait Animal { }
fn main() {
        type ServeFut = /*impl Trait*/;
------------------------------------------
stderr: none



---- [ui] tests/ui/unpretty/box.rs stdout ----
diff of stdout:

2 // check-pass
3 
4 #![feature(stmt_expr_attributes, rustc_attrs)]
+ #![feature(internal_features_lint)]
5 #[prelude_import]
5 #[prelude_import]
6 use ::std::prelude::rust_2015::*;

8 extern crate std;
- 
10 fn main() {
---
To only update this specific test, also pass `--test-args unpretty/box.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unpretty/box.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/box" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/box/auxiliary" "-Zunpretty=hir"
--- stdout -------------------------------
// compile-flags: -Zunpretty=hir
// check-pass

#![feature(stmt_expr_attributes, rustc_attrs)]
#![feature(internal_features_lint)]
#[prelude_import]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
fn main() {
        let _ =
            #[rustc_box]
---

---- [ui] tests/ui/unpretty/bad-literal.rs stdout ----
diff of stdout:

- #[prelude_import]
- use ::std::prelude::rust_2015::*;
- #[macro_use]
- extern crate std;
5 // compile-flags: -Zunpretty=hir
6 // check-fail


8 // In #100948 this caused an ICE with -Zunpretty=hir.
- fn main() {
-         <bad-literal>;
+ #![feature(internal_features_lint)]
+ #![feature(internal_features_lint)]
+ #[prelude_import]
+ use ::std::prelude::rust_2015::*;
+ #[macro_use]
+ extern crate std;
+ fn main() { <bad-literal>; }


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/bad-literal/bad-literal.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/bad-literal/bad-literal.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unpretty/bad-literal.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unpretty/bad-literal.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/bad-literal" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/bad-literal/auxiliary" "-Zunpretty=hir"
--- stdout -------------------------------
// compile-flags: -Zunpretty=hir
// check-fail

// In #100948 this caused an ICE with -Zunpretty=hir.
//~^ ERROR invalid suffix `u` for number literal
#![feature(internal_features_lint)]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
extern crate std;
fn main() { <bad-literal>; }
--- stderr -------------------------------
--- stderr -------------------------------
error: invalid suffix `u` for number literal
  --> fake-test-src-base/unpretty/bad-literal.rs:6:5
LL |     1u;
   |     ^^ invalid suffix `u`
   |
   |
   = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)
error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/unpretty/flattened-format-args.rs stdout ----
diff of stdout:

+ // compile-flags: -Zunpretty=hir -Zflatten-format-args=yes
+ // check-pass
+ 
+ // Should flatten to println!("a 123 b {x} xyz\n"):
+ #![feature(internal_features_lint)]
1 #[prelude_import]
2 use ::std::prelude::rust_2015::*;

4 extern crate std;
4 extern crate std;
- // compile-flags: -Zunpretty=hir -Zflatten-format-args=yes
- // check-pass
8 fn main() {
9         let x = 1;
9         let x = 1;
-         // Should flatten to println!("a 123 b {x} xyz\n"):
11         {
12                 ::std::io::_print(<#[lang = "format_arguments"]>::new_v1(&["a 123 b ",
13                                     " xyz\n"],

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/flattened-format-args/flattened-format-args.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unpretty/flattened-format-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unpretty/flattened-format-args.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/flattened-format-args" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/flattened-format-args/auxiliary" "-Zunpretty=hir" "-Zflatten-format-args=yes"
--- stdout -------------------------------
// compile-flags: -Zunpretty=hir -Zflatten-format-args=yes
// check-pass

// Should flatten to println!("a 123 b {x} xyz\n"):
#![feature(internal_features_lint)]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
fn main() {
        let x = 1;
        {
        {
                ::std::io::_print(<#[lang = "format_arguments"]>::new_v1(&["a 123 b ",
                                    " xyz\n"],
                        &[<#[lang = "format_argument"]>::new_display(&x)]));
    }
------------------------------------------
stderr: none



---- [ui] tests/ui/unpretty/unpretty-expr-fn-arg.rs stdout ----
diff of stdout:

7 // check-pass
8 // compile-flags: -Zunpretty=hir,typed
9 #![allow(dead_code)]
+ 
+ #![feature(internal_features_lint)]
10 #[prelude_import]
10 #[prelude_import]
11 use ::std::prelude::rust_2015::*;

13 extern crate std;
- 
- 
15 fn main() ({ } as ())
- 
17 fn foo((-(128 as i8) as i8)...(127 as i8): i8) ({ } as ())


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/unpretty-expr-fn-arg/unpretty-expr-fn-arg.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/unpretty-expr-fn-arg/unpretty-expr-fn-arg.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unpretty/unpretty-expr-fn-arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unpretty/unpretty-expr-fn-arg.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/unpretty-expr-fn-arg" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/unpretty-expr-fn-arg/auxiliary" "-Zunpretty=hir,typed"
--- stdout -------------------------------
// Regression test for the ICE described in #82328. The pretty-printer for
// `-Zunpretty=hir,typed` would previously retrieve type-checking results
// when entering a body, which means that type information was not available
// for expressions occurring in function signatures, as in the `foo` example
// below, leading to an ICE.
// check-pass
// check-pass
// compile-flags: -Zunpretty=hir,typed
#![allow(dead_code)]

#![feature(internal_features_lint)]
#[prelude_import]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
extern crate std;
fn main() ({ } as ())
fn foo((-(128 as i8) as i8)...(127 as i8): i8) ({ } as ())
stderr: none


---- [ui] tests/ui/unpretty/pretty-let-else.rs stdout ----
---- [ui] tests/ui/unpretty/pretty-let-else.rs stdout ----
diff of stdout:

- #[prelude_import]
- use ::std::prelude::rust_2015::*;
- #[macro_use]
- extern crate std;
5 // compile-flags: -Zunpretty=hir
6 // check-pass

8 
9 
+ 
+ 
+ #![feature(internal_features_lint)]
+ #[prelude_import]
+ use ::std::prelude::rust_2015::*;
+ #[macro_use]
+ extern crate std;
10 fn foo(x:
11         Option<u32>) {
-         let Some(_) = x else
- 
- 
-             { ::std::rt::begin_panic("explicit panic") }
-         };
+         let Some(_) = x else { { ::std::rt::begin_panic("explicit panic") } };
18 fn main() { }
19 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/pretty-let-else/pretty-let-else.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unpretty/pretty-let-else.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unpretty/pretty-let-else.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/pretty-let-else" "-A" "unused" "-A" "internal_features" "-Zcrate-attr=feature(internal_features_lint)" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/pretty-let-else/auxiliary" "-Zunpretty=hir"
--- stdout -------------------------------
// compile-flags: -Zunpretty=hir
// check-pass



#![feature(internal_features_lint)]
#![feature(internal_features_lint)]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
fn foo(x:
        Option<u32>) {
        Option<u32>) {
        let Some(_) = x else { { ::std::rt::begin_panic("explicit panic") } };
fn main() { }
------------------------------------------
stderr: none

