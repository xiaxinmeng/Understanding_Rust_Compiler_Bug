plain
 ---> 2afb3e7bef8f
Step 3/8 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   gcc-multilib   make   ninja-build   file   curl   ca-certificates   python2.7   python3.9   git   cmake   sudo   gdb   llvm-12-tools   llvm-12-dev   libedit-dev   libssl-dev   pkg-config   zlib1g-dev   xz-utils   nodejs
 ---> Using cache
 ---> 357fae1e02d2
Step 4/8 : RUN apt-get update &&     apt-get install -y apt-transport-https software-properties-common &&     curl -s "https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb" > packages-microsoft-prod.deb &&     dpkg -i packages-microsoft-prod.deb &&     apt-get update &&     apt-get install -y powershell
 ---> 080d1843107f
Step 5/8 : COPY scripts/sccache.sh /scripts/
 ---> Using cache
 ---> 4a6c76c56ba3
---

---- [ui] src/test/ui/binop/issue-28837.rs stdout ----
diff of stderr:

112 LL | pub trait BitAnd<Rhs = Self> {
114 
114 
- error[E0369]: no implementation for `A | A`
+ error[E0308]: mismatched types
+    |
+    |
+ LL |     a | a;
+    |         ^ expected struct `core::num::nonzero::NonZero`, found struct `A`
+    |
+    = note: expected struct `core::num::nonzero::NonZero<A>`
+               found struct `A`
+ 
+ error[E0369]: no implementation for `A | core::num::nonzero::NonZero<A>`
117    |
117    |
118 LL |     a | a;

-    |     - ^ - A
+    |     - ^ - core::num::nonzero::NonZero<A>
121    |     A
122    |


- note: an implementation of `BitOr<_>` might be missing for `A`
+ note: the following type would have to `impl` its required traits for this operation to be valid
125    |
126 LL | struct A;


-    | ^^^^^^^^ must implement `BitOr<_>`
-   --> $SRC_DIR/core/src/ops/bit.rs:LL:COL
+    | ^^^^^^^^
+    | |
+    | |
+    | must implement `core::num::nonzero::sealed::ZeroablePrimitive`
+    | must implement `BitOr`
+ note: the following traits must be implemented
+   --> $SRC_DIR/core/src/num/nonzero.rs:LL:COL
130    |
+ LL |     pub trait ZeroablePrimitive: Copy {
+    |
+   ::: $SRC_DIR/core/src/ops/bit.rs:LL:COL
+    |
+    |
131 LL | pub trait BitOr<Rhs = Self> {
133 


277 LL | #[derive(PartialEq, PartialOrd)]
279 
- error: aborting due to 15 previous errors
+ error: aborting due to 16 previous errors
281 
---
To only update this specific test, also pass `--test-args binop/issue-28837.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-28837.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-28837" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-28837/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: cannot add `A` to `A`
   |
   |
LL |     a + a; //~ ERROR cannot add `A` to `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `Add<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Add<_>`
  --> /checkout/library/core/src/ops/arith.rs:100:1
   |
   |
LL | pub trait Add<Rhs = Self> {


error[E0369]: cannot subtract `A` from `A`
   |
   |
LL |     a - a; //~ ERROR cannot subtract `A` from `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `Sub<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Sub<_>`
  --> /checkout/library/core/src/ops/arith.rs:207:1
   |
   |
LL | pub trait Sub<Rhs = Self> {


error[E0369]: cannot multiply `A` by `A`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     a * a; //~ ERROR cannot multiply `A` by `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `Mul<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Mul<_>`
  --> /checkout/library/core/src/ops/arith.rs:336:1
   |
   |
LL | pub trait Mul<Rhs = Self> {

error[E0369]: cannot divide `A` by `A`
  --> /checkout/src/test/ui/binop/issue-28837.rs:12:7
   |
   |
LL |     a / a; //~ ERROR cannot divide `A` by `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `Div<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Div<_>`
  --> /checkout/library/core/src/ops/arith.rs:469:1
   |
   |
LL | pub trait Div<Rhs = Self> {


error[E0369]: cannot mod `A` by `A`
   |
   |
LL |     a % a; //~ ERROR cannot mod `A` by `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `Rem<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Rem<_>`
  --> /checkout/library/core/src/ops/arith.rs:571:1
   |
   |
LL | pub trait Rem<Rhs = Self> {


error[E0369]: no implementation for `A & A`
   |
   |
LL |     a & a; //~ ERROR no implementation for `A & A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `BitAnd<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `BitAnd<_>`
  --> /checkout/library/core/src/ops/bit.rs:146:1
   |
   |
LL | pub trait BitAnd<Rhs = Self> {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/binop/issue-28837.rs:18:9
   |
   |
LL |     a | a; //~ ERROR no implementation for `A | A`
   |         ^ expected struct `core::num::nonzero::NonZero`, found struct `A`
   |
   = note: expected struct `core::num::nonzero::NonZero<A>`
              found struct `A`

error[E0369]: no implementation for `A | core::num::nonzero::NonZero<A>`
   |
   |
LL |     a | a; //~ ERROR no implementation for `A | A`
   |     - ^ - core::num::nonzero::NonZero<A>
   |     A
   |
   |
note: the following type would have to `impl` its required traits for this operation to be valid
   |
LL | struct A;
   | ^^^^^^^^
   | |
   | |
   | must implement `core::num::nonzero::sealed::ZeroablePrimitive`
   | must implement `BitOr`
  --> /checkout/library/core/src/num/nonzero.rs:20:5
   |
   |
LL |     pub trait ZeroablePrimitive: Copy {
   |
  ::: /checkout/library/core/src/ops/bit.rs:247:1
   |
   |
LL | pub trait BitOr<Rhs = Self> {


error[E0369]: no implementation for `A << A`
   |
   |
LL |     a << a; //~ ERROR no implementation for `A << A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `Shl<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Shl<_>`
  --> /checkout/library/core/src/ops/bit.rs:448:1
   |
   |
LL | pub trait Shl<Rhs = Self> {


error[E0369]: no implementation for `A >> A`
   |
   |
LL |     a >> a; //~ ERROR no implementation for `A >> A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `Shr<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Shr<_>`
  --> /checkout/library/core/src/ops/bit.rs:567:1
   |
   |
LL | pub trait Shr<Rhs = Self> {


error[E0369]: binary operation `==` cannot be applied to type `A`
   |
   |
LL |     a == a; //~ ERROR binary operation `==` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `PartialEq<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialEq<_>`
help: consider annotating `A` with `#[derive(PartialEq)]`
   |
LL | #[derive(PartialEq)]


error[E0369]: binary operation `!=` cannot be applied to type `A`
   |
   |
LL |     a != a; //~ ERROR binary operation `!=` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `PartialEq<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialEq<_>`
help: consider annotating `A` with `#[derive(PartialEq)]`
   |
LL | #[derive(PartialEq)]


error[E0369]: binary operation `<` cannot be applied to type `A`
   |
   |
LL |     a < a; //~ ERROR binary operation `<` cannot be applied to type `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `PartialOrd<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialOrd<_>`
help: consider annotating `A` with `#[derive(PartialEq, PartialOrd)]`
   |
LL | #[derive(PartialEq, PartialOrd)]


error[E0369]: binary operation `<=` cannot be applied to type `A`
   |
   |
LL |     a <= a; //~ ERROR binary operation `<=` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `PartialOrd<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialOrd<_>`
help: consider annotating `A` with `#[derive(PartialEq, PartialOrd)]`
   |
LL | #[derive(PartialEq, PartialOrd)]


error[E0369]: binary operation `>` cannot be applied to type `A`
   |
   |
LL |     a > a; //~ ERROR binary operation `>` cannot be applied to type `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `PartialOrd<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialOrd<_>`
help: consider annotating `A` with `#[derive(PartialEq, PartialOrd)]`
   |
LL | #[derive(PartialEq, PartialOrd)]


error[E0369]: binary operation `>=` cannot be applied to type `A`
   |
   |
LL |     a >= a; //~ ERROR binary operation `>=` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `PartialOrd<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialOrd<_>`
help: consider annotating `A` with `#[derive(PartialEq, PartialOrd)]`
   |
LL | #[derive(PartialEq, PartialOrd)]

error: aborting due to 16 previous errors

Some errors have detailed explanations: E0308, E0369.
Some errors have detailed explanations: E0308, E0369.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/fmt/ifmt-unimpl.rs stdout ----
diff of stderr:

7    = help: the following other types implement trait `UpperHex`:
8              &T
-              NonZeroI128
-              NonZeroI16
-              NonZeroI32
-              NonZeroI64
---
+              core::num::nonzero::NonZero<T>
+              i128
+              i16
+            and 10 others
17    = note: required because of the requirements on the impl of `UpperHex` for `&str`
18 note: required by a bound in `ArgumentV1::<'a>::new_upper_hex`
19   --> $SRC_DIR/core/src/fmt/mod.rs:LL:COL

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl/ifmt-unimpl.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/ifmt-unimpl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/ifmt-unimpl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: UpperHex` is not satisfied
   |
   |
LL |     format!("{:X}", "3");
   |                     ^^^ the trait `UpperHex` is not implemented for `str`
   |
   = help: the following other types implement trait `UpperHex`:
             &T
             Saturating<T>
             Simd<T, LANES>
             Wrapping<T>
             core::num::nonzero::NonZero<T>
             core::num::nonzero::NonZero<T>
             i128
             i16
           and 10 others
   = note: required because of the requirements on the impl of `UpperHex` for `&str`
note: required by a bound in `ArgumentV1::<'a>::new_upper_hex`
   |
   |
LL |     arg_new!(new_upper_hex, UpperHex);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ArgumentV1::<'a>::new_upper_hex`
   = note: this error originates in the macro `$crate::__export::format_args` which comes from the expansion of the macro `arg_new` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/layout/zero-sized-array-enum-niche.rs stdout ----
diff of stderr:

218 LL | enum MultipleAlignments {
220 
220 
- error: layout_of(std::result::Result<[u32; 0], Packed<std::num::NonZeroU16>>) = Layout {
+ error: layout_of(std::result::Result<[u32; 0], Packed<core::num::nonzero::NonZero<u16>>>) = Layout {
222            size: Size(4 bytes),
223            align: AbiAndPrefAlign {
224                abi: Align(4 bytes),

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/zero-sized-array-enum-niche/zero-sized-array-enum-niche.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args layout/zero-sized-array-enum-niche.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/zero-sized-array-enum-niche.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/zero-sized-array-enum-niche" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/zero-sized-array-enum-niche/auxiliary"
stdout: none
--- stderr -------------------------------
error: layout_of(std::result::Result<[u32; 0], bool>) = Layout {
           size: Size(4 bytes),
           align: AbiAndPrefAlign {
               abi: Align(4 bytes),
               pref: Align(8 bytes),
           abi: Aggregate {
               sized: true,
           },
           fields: Arbitrary {
           fields: Arbitrary {
               offsets: [
                   Size(0 bytes),
               ],
               memory_index: [
                   0,
               ],
           },
           largest_niche: Some(
               Niche {
                   offset: Size(0 bytes),
                   value: Int(
                       false,
                   ),
                   valid_range: 0..=1,
               },
---
               tag_encoding: Direct,
               tag_field: 0,
               variants: [
                   Layout {
                       size: Size(4 bytes),
                       align: AbiAndPrefAlign {
                           abi: Align(4 bytes),
                           pref: Align(8 bytes),
                       abi: Aggregate {
                           sized: true,
                       },
                       fields: Arbitrary {
---
                           index: 0,
                       },
                   },
                   Layout {
                       size: Size(2 bytes),
                       align: AbiAndPrefAlign {
                           abi: Align(1 bytes),
                           pref: Align(8 bytes),
                       abi: Aggregate {
                           sized: true,
                       },
                       fields: Arbitrary {
                       fields: Arbitrary {
                           offsets: [
                               Size(1 bytes),
                           ],
                           memory_index: [
                               0,
                           ],
                       },
                       largest_niche: Some(
                           Niche {
                               offset: Size(1 bytes),
                               value: Int(
                                   false,
                               ),
                               valid_range: 0..=1,
                           },
---
           },
       }
  --> /checkout/src/test/ui/layout/zero-sized-array-enum-niche.rs:13:1
   |
LL | type AlignedResult = Result<[u32; 0], bool>; //~ ERROR: layout_of


error: layout_of(MultipleAlignments) = Layout {
           size: Size(4 bytes),
           align: AbiAndPrefAlign {
               abi: Align(4 bytes),
               pref: Align(8 bytes),
           abi: Aggregate {
               sized: true,
           },
           fields: Arbitrary {
           fields: Arbitrary {
               offsets: [
                   Size(0 bytes),
               ],
               memory_index: [
                   0,
               ],
           },
           largest_niche: Some(
               Niche {
                   offset: Size(0 bytes),
                   value: Int(
                       false,
                   ),
                   valid_range: 0..=2,
               },
---
               tag_encoding: Direct,
               tag_field: 0,
               variants: [
                   Layout {
                       size: Size(2 bytes),
                       align: AbiAndPrefAlign {
                           abi: Align(2 bytes),
                           pref: Align(8 bytes),
                       abi: Aggregate {
                           sized: true,
                       },
                       fields: Arbitrary {
---
                           index: 0,
                       },
                   },
                   Layout {
                       size: Size(4 bytes),
                       align: AbiAndPrefAlign {
                           abi: Align(4 bytes),
                           pref: Align(8 bytes),
                       abi: Aggregate {
                           sized: true,
                       },
                       fields: Arbitrary {
---
                           index: 1,
                       },
                   },
                   Layout {
                       size: Size(2 bytes),
                       align: AbiAndPrefAlign {
                           abi: Align(1 bytes),
                           pref: Align(8 bytes),
                       abi: Aggregate {
                           sized: true,
                       },
                       fields: Arbitrary {
                       fields: Arbitrary {
                           offsets: [
                               Size(1 bytes),
                           ],
                           memory_index: [
                               0,
                           ],
                       },
                       largest_niche: Some(
                           Niche {
                               offset: Size(1 bytes),
                               value: Int(
                                   false,
                               ),
                               valid_range: 0..=1,
                           },
---
           },
       }
  --> /checkout/src/test/ui/layout/zero-sized-array-enum-niche.rs:21:1
   |
LL | enum MultipleAlignments { //~ ERROR: layout_of


error: layout_of(std::result::Result<[u32; 0], Packed<core::num::nonzero::NonZero<u16>>>) = Layout {
           size: Size(4 bytes),
           align: AbiAndPrefAlign {
               abi: Align(4 bytes),
               pref: Align(8 bytes),
           abi: Aggregate {
               sized: true,
           },
           fields: Arbitrary {
           fields: Arbitrary {
               offsets: [
                   Size(0 bytes),
               ],
               memory_index: [
                   0,
               ],
           },
           largest_niche: Some(
               Niche {
                   offset: Size(0 bytes),
                   value: Int(
                       false,
                   ),
                   valid_range: 0..=1,
               },
---
               tag_encoding: Direct,
               tag_field: 0,
               variants: [
                   Layout {
                       size: Size(4 bytes),
                       align: AbiAndPrefAlign {
                           abi: Align(4 bytes),
                           pref: Align(8 bytes),
                       abi: Aggregate {
                           sized: true,
                       },
                       fields: Arbitrary {
---
                           index: 0,
                       },
                   },
                   Layout {
                       size: Size(3 bytes),
                       align: AbiAndPrefAlign {
                           abi: Align(1 bytes),
                           pref: Align(8 bytes),
                       abi: Aggregate {
                           sized: true,
                       },
                       fields: Arbitrary {
                       fields: Arbitrary {
                           offsets: [
                               Size(1 bytes),
                           ],
                           memory_index: [
                               0,
                           ],
                       },
                       largest_niche: Some(
                           Niche {
                               offset: Size(1 bytes),
                               value: Int(
                                   false,
                               ),
                               valid_range: 1..=65535,
                           },
---
           },
       }
  --> /checkout/src/test/ui/layout/zero-sized-array-enum-niche.rs:37:1
   |
LL | type NicheLosesToTagged = Result<[u32; 0], Packed<std::num::NonZeroU16>>; //~ ERROR: layout_of


error: layout_of(std::result::Result<[u32; 0], Packed<U16IsZero>>) = Layout {
           size: Size(4 bytes),
           align: AbiAndPrefAlign {
               abi: Align(4 bytes),
               pref: Align(8 bytes),
           abi: Aggregate {
               sized: true,
           },
           fields: Arbitrary {
           fields: Arbitrary {
               offsets: [
                   Size(0 bytes),
               ],
               memory_index: [
                   0,
               ],
           },
           largest_niche: Some(
               Niche {
                   offset: Size(0 bytes),
                   value: Int(
                       false,
                   ),
                   valid_range: 0..=1,
               },
---
                       false,
                   ),
                   valid_range: 0..=1,
               },
               tag_encoding: Niche {
                   dataful_variant: 1,
                   niche_variants: 0..=0,
                   niche_start: 1,
               tag_field: 0,
               variants: [
                   Layout {
                       size: Size(0 bytes),
                       size: Size(0 bytes),
                       align: AbiAndPrefAlign {
                           abi: Align(4 bytes),
                           pref: Align(8 bytes),
                       abi: Aggregate {
                           sized: true,
                       },
                       fields: Arbitrary {
---
                           index: 0,
                       },
                   },
                   Layout {
                       size: Size(2 bytes),
                       align: AbiAndPrefAlign {
                           abi: Align(1 bytes),
                           pref: Align(8 bytes),
                       abi: Aggregate {
                           sized: true,
                       },
                       fields: Arbitrary {
                       fields: Arbitrary {
                           offsets: [
                               Size(0 bytes),
                           ],
                           memory_index: [
                               0,
                           ],
                       },
                       largest_niche: Some(
                           Niche {
                               offset: Size(0 bytes),
                               value: Int(
                                   false,
                               ),
                               valid_range: 0..=0,
                           },
---
           },
       }
  --> /checkout/src/test/ui/layout/zero-sized-array-enum-niche.rs:44:1
   |
LL | type NicheWinsOverTagged = Result<[u32; 0], Packed<U16IsZero>>; //~ ERROR: layout_of

error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/lint/clashing-extern-fn.rs stdout ----
diff of stderr:

150 LL |             fn non_zero_usize() -> usize;
151    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
152    |
-    = note: expected `unsafe extern "C" fn() -> NonZeroUsize`
+    = note: expected `unsafe extern "C" fn() -> NonZero<usize>`
154               found `unsafe extern "C" fn() -> usize`
156 warning: `non_null_ptr` redeclared with a different signature

211    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
212    |
212    |
213    = note: expected `unsafe extern "C" fn() -> usize`
-               found `unsafe extern "C" fn() -> Option<UnsafeCell<NonZeroUsize>>`
+               found `unsafe extern "C" fn() -> Option<UnsafeCell<NonZero<usize>>>`
215 
216 warning: `extern` block uses type `Option<TransparentNoNiche>`, which is not FFI-safe


223    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
224    = note: enum has no representation hint
225 
- warning: `extern` block uses type `Option<UnsafeCell<NonZeroUsize>>`, which is not FFI-safe
+ warning: `extern` block uses type `Option<UnsafeCell<NonZero<usize>>>`, which is not FFI-safe
228    |
228    |
229 LL |             fn hidden_niche_unsafe_cell() -> Option<UnsafeCell<NonZeroUsize>>;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/clashing-extern-fn/clashing-extern-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/clashing-extern-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/clashing-extern-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/clashing-extern-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/clashing-extern-fn/auxiliary"
stdout: none
--- stderr -------------------------------
warning: `clash` redeclared with a different signature
   |
   |
LL |             fn clash(x: u8);
   |             ---------------- `clash` previously declared here
...
LL |             fn clash(x: u64); //~ WARN `clash` redeclared with a different signature
   |             ^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:4:9
   |
LL | #![warn(clashing_extern_declarations)]
LL | #![warn(clashing_extern_declarations)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `unsafe extern "C" fn(u8)`
              found `unsafe extern "C" fn(u64)`

warning: `extern_link_name` redeclared with a different signature
   |
LL | /     #[link_name = "extern_link_name"]
LL | /     #[link_name = "extern_link_name"]
LL | |     fn some_new_name(x: i16);
   | |_____________________________- `extern_link_name` previously declared here
...
LL |           fn extern_link_name(x: u32);
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(i16)`
   = note: expected `unsafe extern "C" fn(i16)`
              found `unsafe extern "C" fn(u32)`

warning: `some_other_extern_link_name` redeclares `some_other_new_name` with a different signature
   |
   |
LL |       fn some_other_new_name(x: i16);
   |       ------------------------------- `some_other_new_name` previously declared here
...
LL | /         #[link_name = "some_other_new_name"]
LL | |         //~^ WARN `some_other_extern_link_name` redeclares `some_other_new_name` with a different
LL | |         fn some_other_extern_link_name(x: u32);
   | |_______________________________________________^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(i16)`
   = note: expected `unsafe extern "C" fn(i16)`
              found `unsafe extern "C" fn(u32)`

warning: `other_both_names_different` redeclares `link_name_same` with a different signature
   |
LL | /     #[link_name = "link_name_same"]
LL | /     #[link_name = "link_name_same"]
LL | |     fn both_names_different(x: i16);
   | |____________________________________- `link_name_same` previously declared here
LL | /         #[link_name = "link_name_same"]
LL | /         #[link_name = "link_name_same"]
LL | |         //~^ WARN `other_both_names_different` redeclares `link_name_same` with a different
LL | |         fn other_both_names_different(x: u32);
   | |______________________________________________^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn(i16)`
   = note: expected `unsafe extern "C" fn(i16)`
              found `unsafe extern "C" fn(u32)`

warning: `different_mod` redeclared with a different signature
   |
   |
LL |         fn different_mod(x: u8);
   |         ------------------------ `different_mod` previously declared here
...
LL |         fn different_mod(x: u64); //~ WARN `different_mod` redeclared with a different signature
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(u8)`
              found `unsafe extern "C" fn(u64)`

warning: `variadic_decl` redeclared with a different signature
   |
   |
LL |     fn variadic_decl(x: u8, ...);
   |     ----------------------------- `variadic_decl` previously declared here
...
LL |         fn variadic_decl(x: u8); //~ WARN `variadic_decl` redeclared with a different signature
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(u8, ...)`
              found `unsafe extern "C" fn(u8)`

warning: `weigh_banana` redeclared with a different signature
   |
   |
LL |             fn weigh_banana(count: *const Banana) -> u64;
   |             --------------------------------------------- `weigh_banana` previously declared here
...
LL |             fn weigh_banana(count: *const Banana) -> u64;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(*const one::Banana) -> u64`
              found `unsafe extern "C" fn(*const three::Banana) -> u64`

warning: `draw_point` redeclared with a different signature
   |
   |
LL |             fn draw_point(p: Point);
   |             ------------------------ `draw_point` previously declared here
...
LL |             fn draw_point(p: Point);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn(sameish_members::a::Point)`
              found `unsafe extern "C" fn(sameish_members::b::Point)`

warning: `origin` redeclared with a different signature
   |
LL |             fn origin() -> Point3;
LL |             fn origin() -> Point3;
   |             ---------------------- `origin` previously declared here
...
LL |             fn origin() -> Point3; //~ WARN `origin` redeclared with a different signature
   |             ^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn() -> same_sized_members_clash::a::Point3`
              found `unsafe extern "C" fn() -> same_sized_members_clash::b::Point3`

warning: `transparent_incorrect` redeclared with a different signature
   |
LL |             fn transparent_incorrect() -> T;
LL |             fn transparent_incorrect() -> T;
   |             -------------------------------- `transparent_incorrect` previously declared here
LL |             fn transparent_incorrect() -> isize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn() -> T`
   = note: expected `unsafe extern "C" fn() -> T`
              found `unsafe extern "C" fn() -> isize`

warning: `missing_return_type` redeclared with a different signature
   |
LL |             fn missing_return_type() -> usize;
LL |             fn missing_return_type() -> usize;
   |             ---------------------------------- `missing_return_type` previously declared here
LL |             fn missing_return_type();
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn() -> usize`
   = note: expected `unsafe extern "C" fn() -> usize`
              found `unsafe extern "C" fn()`

warning: `non_zero_usize` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:256:13
   |
LL |             fn non_zero_usize() -> core::num::NonZeroUsize;
   |             ----------------------------------------------- `non_zero_usize` previously declared here
LL |             fn non_zero_usize() -> usize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   |
   = note: expected `unsafe extern "C" fn() -> NonZero<usize>`
              found `unsafe extern "C" fn() -> usize`
warning: `non_null_ptr` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:258:13
   |
   |
LL |             fn non_null_ptr() -> core::ptr::NonNull<usize>;
   |             ----------------------------------------------- `non_null_ptr` previously declared here
LL |             fn non_null_ptr() -> *const usize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   |
   = note: expected `unsafe extern "C" fn() -> NonNull<usize>`
              found `unsafe extern "C" fn() -> *const usize`
warning: `option_non_zero_usize_incorrect` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:356:13
   |
LL |             fn option_non_zero_usize_incorrect() -> usize;
LL |             fn option_non_zero_usize_incorrect() -> usize;
   |             ---------------------------------------------- `option_non_zero_usize_incorrect` previously declared here
LL |             fn option_non_zero_usize_incorrect() -> isize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn() -> usize`
   = note: expected `unsafe extern "C" fn() -> usize`
              found `unsafe extern "C" fn() -> isize`

warning: `option_non_null_ptr_incorrect` redeclared with a different signature
  --> /checkout/src/test/ui/lint/clashing-extern-fn.rs:358:13
   |
LL |             fn option_non_null_ptr_incorrect() -> *const usize;
   |             --------------------------------------------------- `option_non_null_ptr_incorrect` previously declared here
LL |             fn option_non_null_ptr_incorrect() -> *const isize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   |
   = note: expected `unsafe extern "C" fn() -> *const usize`
   = note: expected `unsafe extern "C" fn() -> *const usize`
              found `unsafe extern "C" fn() -> *const isize`

warning: `hidden_niche_transparent_no_niche` redeclared with a different signature
   |
   |
LL |             fn hidden_niche_transparent_no_niche() -> usize;
   |             ------------------------------------------------ `hidden_niche_transparent_no_niche` previously declared here
...
LL |             fn hidden_niche_transparent_no_niche() -> Option<TransparentNoNiche>;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn() -> usize`
   = note: expected `unsafe extern "C" fn() -> usize`
              found `unsafe extern "C" fn() -> Option<TransparentNoNiche>`

warning: `hidden_niche_unsafe_cell` redeclared with a different signature
   |
   |
LL |             fn hidden_niche_unsafe_cell() -> usize;
   |             --------------------------------------- `hidden_niche_unsafe_cell` previously declared here
...
LL |             fn hidden_niche_unsafe_cell() -> Option<UnsafeCell<NonZeroUsize>>;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
   = note: expected `unsafe extern "C" fn() -> usize`
   = note: expected `unsafe extern "C" fn() -> usize`
              found `unsafe extern "C" fn() -> Option<UnsafeCell<NonZero<usize>>>`

warning: `extern` block uses type `Option<TransparentNoNiche>`, which is not FFI-safe
   |
   |
LL |             fn hidden_niche_transparent_no_niche() -> Option<TransparentNoNiche>;
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   = note: `#[warn(improper_ctypes)]` on by default
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint

warning: `extern` block uses type `Option<UnsafeCell<NonZero<usize>>>`, which is not FFI-safe
   |
   |
LL |             fn hidden_niche_unsafe_cell() -> Option<UnsafeCell<NonZeroUsize>>;
   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint
warning: 19 warnings emitted
------------------------------------------



---- [ui] src/test/ui/lint/lint-ctypes-enum.rs stdout ----
diff of stderr:

61    |
62    = note: 128-bit integers don't currently have a known stable ABI
63 
- error: `extern` block uses type `Option<TransparentUnion<NonZeroU8>>`, which is not FFI-safe
+ error: `extern` block uses type `Option<TransparentUnion<core::num::nonzero::NonZero<u8>>>`, which is not FFI-safe
66    |
66    |
67 LL |    fn transparent_union(x: Option<TransparentUnion<num::NonZeroU8>>);

70    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
71    = note: enum has no representation hint
72 
- error: `extern` block uses type `Option<Rust<NonZeroU8>>`, which is not FFI-safe
+ error: `extern` block uses type `Option<Rust<core::num::nonzero::NonZero<u8>>>`, which is not FFI-safe
75    |
75    |
76 LL |    fn repr_rust(x: Option<Rust<num::NonZeroU8>>);

79    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
80    = note: enum has no representation hint
81 
- error: `extern` block uses type `Result<(), NonZeroI32>`, which is not FFI-safe
+ error: `extern` block uses type `Result<(), core::num::nonzero::NonZero<i32>>`, which is not FFI-safe
84    |
84    |
85 LL |    fn no_result(x: Result<(), num::NonZeroI32>);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum/lint-ctypes-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-ctypes-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum/auxiliary"
stdout: none
--- stderr -------------------------------
error: `extern` block uses type `U`, which is not FFI-safe
   |
   |
LL |    fn uf(x: U); //~ ERROR `extern` block uses type `U`
   |             ^ not FFI-safe
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:3:9
   |
   |
LL | #![deny(improper_ctypes)]
   |         ^^^^^^^^^^^^^^^
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint
  --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:9:1
   |
   |
LL | enum U {


error: `extern` block uses type `B`, which is not FFI-safe
   |
   |
LL |    fn bf(x: B); //~ ERROR `extern` block uses type `B`
   |             ^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint
  --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:12:1
   |
   |
LL | enum B {


error: `extern` block uses type `T`, which is not FFI-safe
   |
   |
LL |    fn tf(x: T); //~ ERROR `extern` block uses type `T`
   |             ^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint
  --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:16:1
   |
   |
LL | enum T {


error: `extern` block uses type `u128`, which is not FFI-safe
   |
   |
LL |    fn nonzero_u128(x: Option<num::NonZeroU128>);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = note: 128-bit integers don't currently have a known stable ABI

error: `extern` block uses type `i128`, which is not FFI-safe
   |
   |
LL |    fn nonzero_i128(x: Option<num::NonZeroI128>);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = note: 128-bit integers don't currently have a known stable ABI

error: `extern` block uses type `Option<TransparentUnion<core::num::nonzero::NonZero<u8>>>`, which is not FFI-safe
   |
   |
LL |    fn transparent_union(x: Option<TransparentUnion<num::NonZeroU8>>);
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint

error: `extern` block uses type `Option<Rust<core::num::nonzero::NonZero<u8>>>`, which is not FFI-safe
   |
   |
LL |    fn repr_rust(x: Option<Rust<num::NonZeroU8>>); //~ ERROR `extern` block uses type
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint

error: `extern` block uses type `Result<(), core::num::nonzero::NonZero<i32>>`, which is not FFI-safe
   |
   |
LL |    fn no_result(x: Result<(), num::NonZeroI32>); //~ ERROR `extern` block uses type
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint
error: aborting due to 8 previous errors
------------------------------------------



---- [ui] src/test/ui/lint/uninitialized-zeroed.rs stdout ----
diff of stderr:

436    |
437    = note: references must be non-null
438 
- error: the type `NonZeroU32` does not permit zero-initialization
+ error: the type `core::num::nonzero::NonZero<u32>` does not permit zero-initialization
441    |
442 LL |         let _val: NonZeroU32 = mem::transmute(0);

445    |                                this code causes undefined behavior when executed
445    |                                this code causes undefined behavior when executed
446    |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
-    = note: `std::num::NonZeroU32` must be non-null
-    = note: `std::num::NonZeroU32` must be non-null
+    = note: `core::num::nonzero::NonZero<u32>` must be non-null
449 
450 error: the type `NonNull<i32>` does not permit zero-initialization


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/uninitialized-zeroed.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/uninitialized-zeroed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/uninitialized-zeroed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/uninitialized-zeroed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/auxiliary"
stdout: none
--- stderr -------------------------------
error: the type `&T` does not permit zero-initialization
   |
   |
LL |         let _val: &'static T = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                |
   |                                this code causes undefined behavior when executed
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:6:9
   |
LL | #![deny(invalid_value)]
LL | #![deny(invalid_value)]
   |         ^^^^^^^^^^^^^
   = note: references must be non-null

error: the type `&T` does not permit being left uninitialized
   |
   |
LL |         let _val: &'static T = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                |
   |                                this code causes undefined behavior when executed
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: references must be non-null


error: the type `Wrap<&T>` does not permit zero-initialization
   |
   |
LL |         let _val: Wrap<&'static T> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                      |
   |                                      this code causes undefined behavior when executed
   |                                      this code causes undefined behavior when executed
   |                                      help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:17:18
   |
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `Wrap<&T>` does not permit being left uninitialized
---
To only update this specific test, also pass `--test-args or-patterns/or-patterns-syntactic-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error: top-level or-patterns are not allowed in function parameters
   |
   |
LL |     fn fun1(A | B: E) {}
   |             ^^^^^ help: wrap the pattern in parentheses: `(A | B)`
error: top-level or-patterns are not allowed in function parameters
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:18:13
   |
   |
LL |     fn fun2(| A | B: E) {}
   |             ^^^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: top-level or-patterns are not allowed in `let` bindings
   |
   |
LL |     let A | B: E = A;
   |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: top-level or-patterns are not allowed in `let` bindings
   |
   |
LL |     let | A | B: E = A;
   |         ^^^^^^^ help: wrap the pattern in parentheses: `(A | B)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:11:24
   |
   |
LL |     let _ = |A | B: E| (); //~ ERROR no implementation for `E | ()`
   |                        ^^ expected struct `core::num::nonzero::NonZero`, found `()`
   |
   = note: expected struct `core::num::nonzero::NonZero<E>`


error[E0369]: no implementation for `E | core::num::nonzero::NonZero<E>`
   |
   |
LL |     let _ = |A | B: E| (); //~ ERROR no implementation for `E | ()`
   |                  ----^ -- core::num::nonzero::NonZero<E>
   |                  E
   |
   |
note: the following type would have to `impl` its required traits for this operation to be valid
   |
   |
LL | enum E { A, B }
   | |
   | |
   | must implement `core::num::nonzero::sealed::ZeroablePrimitive`
   | must implement `BitOr`
  --> /checkout/library/core/src/num/nonzero.rs:20:5
   |
   |
LL |     pub trait ZeroablePrimitive: Copy {
   |
  ::: /checkout/library/core/src/ops/bit.rs:247:1
   |
   |
LL | pub trait BitOr<Rhs = Self> {

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0369.
Some errors have detailed explanations: E0308, E0369.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/print_type_sizes/niche-filling.rs stdout ----
diff of stdout:

14 print-type-size         field `.post`: 2 bytes
15 print-type-size         field `.val`: 4 bytes
16 print-type-size     variant `None`: 0 bytes
- print-type-size type: `MyOption<Union1<std::num::NonZeroU32>>`: 8 bytes, alignment: 4 bytes
+ print-type-size type: `MyOption<Union1<core::num::nonzero::NonZero<u32>>>`: 8 bytes, alignment: 4 bytes
18 print-type-size     discriminant: 4 bytes
19 print-type-size     variant `Some`: 4 bytes
20 print-type-size         field `.0`: 4 bytes
21 print-type-size     variant `None`: 0 bytes
21 print-type-size     variant `None`: 0 bytes
- print-type-size type: `MyOption<Union2<std::num::NonZeroU32, std::num::NonZeroU32>>`: 8 bytes, alignment: 4 bytes
+ print-type-size type: `MyOption<Union2<core::num::nonzero::NonZero<u32>, core::num::nonzero::NonZero<u32>>>`: 8 bytes, alignment: 4 bytes
23 print-type-size     discriminant: 4 bytes
24 print-type-size     variant `Some`: 4 bytes
25 print-type-size         field `.0`: 4 bytes
26 print-type-size     variant `None`: 0 bytes
26 print-type-size     variant `None`: 0 bytes
- print-type-size type: `MyOption<Union2<std::num::NonZeroU32, u32>>`: 8 bytes, alignment: 4 bytes
+ print-type-size type: `MyOption<Union2<core::num::nonzero::NonZero<u32>, u32>>`: 8 bytes, alignment: 4 bytes
28 print-type-size     discriminant: 4 bytes
29 print-type-size     variant `Some`: 4 bytes
30 print-type-size         field `.0`: 4 bytes
53 print-type-size     variant `Some`: 4 bytes
53 print-type-size     variant `Some`: 4 bytes
54 print-type-size         field `.0`: 4 bytes
55 print-type-size     variant `None`: 0 bytes
- print-type-size type: `MyOption<std::num::NonZeroU32>`: 4 bytes, alignment: 4 bytes
+ print-type-size type: `MyOption<core::num::nonzero::NonZero<u32>>`: 4 bytes, alignment: 4 bytes
57 print-type-size     variant `Some`: 4 bytes
58 print-type-size         field `.0`: 4 bytes
59 print-type-size     variant `None`: 0 bytes

- print-type-size type: `Union1<std::num::NonZeroU32>`: 4 bytes, alignment: 4 bytes
+ print-type-size type: `Union1<core::num::nonzero::NonZero<u32>>`: 4 bytes, alignment: 4 bytes
61 print-type-size     variant `Union1`: 4 bytes
62 print-type-size         field `.a`: 4 bytes
- print-type-size type: `Union2<std::num::NonZeroU32, std::num::NonZeroU32>`: 4 bytes, alignment: 4 bytes
+ print-type-size type: `Union2<core::num::nonzero::NonZero<u32>, core::num::nonzero::NonZero<u32>>`: 4 bytes, alignment: 4 bytes
64 print-type-size     variant `Union2`: 4 bytes
65 print-type-size         field `.a`: 4 bytes
66 print-type-size         field `.b`: 4 bytes, offset: 0 bytes, alignment: 4 bytes

- print-type-size type: `Union2<std::num::NonZeroU32, u32>`: 4 bytes, alignment: 4 bytes
+ print-type-size type: `Union2<core::num::nonzero::NonZero<u32>, u32>`: 4 bytes, alignment: 4 bytes
68 print-type-size     variant `Union2`: 4 bytes
69 print-type-size         field `.a`: 4 bytes
70 print-type-size         field `.b`: 4 bytes, offset: 0 bytes, alignment: 4 bytes

- print-type-size type: `std::num::NonZeroU32`: 4 bytes, alignment: 4 bytes
+ print-type-size type: `core::num::nonzero::NonZero<u32>`: 4 bytes, alignment: 4 bytes
72 print-type-size     field `.0`: 4 bytes
73 print-type-size type: `Enum4<(), (), (), MyOption<u8>>`: 2 bytes, alignment: 1 bytes
74 print-type-size     variant `Four`: 2 bytes

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling/niche-filling.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/niche-filling.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print_type_sizes/niche-filling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "print-type-sizes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling/auxiliary"
--- stdout -------------------------------
print-type-size type: `IndirectNonZero`: 12 bytes, alignment: 4 bytes
print-type-size     field `.nested`: 8 bytes
print-type-size     field `.post`: 2 bytes
print-type-size     field `.pre`: 1 bytes
print-type-size     end padding: 1 bytes
print-type-size type: `MyOption<IndirectNonZero>`: 12 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 12 bytes
print-type-size         field `.0`: 12 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `EmbeddedDiscr`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Record`: 7 bytes
print-type-size         field `.pre`: 1 bytes
print-type-size         field `.post`: 2 bytes
print-type-size         field `.val`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<Union1<core::num::nonzero::NonZero<u32>>>`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<Union2<core::num::nonzero::NonZero<u32>, core::num::nonzero::NonZero<u32>>>`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<Union2<core::num::nonzero::NonZero<u32>, u32>>`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `NestedNonZero`: 8 bytes, alignment: 4 bytes
print-type-size     field `.val`: 4 bytes
print-type-size     field `.post`: 2 bytes
print-type-size     field `.pre`: 1 bytes
print-type-size     end padding: 1 bytes
print-type-size type: `Enum4<(), char, (), ()>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Two`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `One`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Three`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Four`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size type: `MyNotNegativeOne`: 4 bytes, alignment: 4 bytes
print-type-size     field `._i`: 4 bytes
print-type-size type: `MyOption<MyNotNegativeOne>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<char>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<core::num::nonzero::NonZero<u32>>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `Union1<core::num::nonzero::NonZero<u32>>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Union1`: 4 bytes
print-type-size         field `.a`: 4 bytes
print-type-size type: `Union2<core::num::nonzero::NonZero<u32>, core::num::nonzero::NonZero<u32>>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Union2`: 4 bytes
print-type-size         field `.a`: 4 bytes
print-type-size         field `.b`: 4 bytes, offset: 0 bytes, alignment: 4 bytes
print-type-size type: `Union2<core::num::nonzero::NonZero<u32>, u32>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Union2`: 4 bytes
print-type-size         field `.a`: 4 bytes
print-type-size         field `.b`: 4 bytes, offset: 0 bytes, alignment: 4 bytes
print-type-size type: `core::num::nonzero::NonZero<u32>`: 4 bytes, alignment: 4 bytes
print-type-size     field `.0`: 4 bytes
print-type-size type: `Enum4<(), (), (), MyOption<u8>>`: 2 bytes, alignment: 1 bytes
print-type-size     variant `Four`: 2 bytes
print-type-size         field `.0`: 2 bytes
print-type-size     variant `One`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Two`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Three`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size type: `MyOption<MyOption<u8>>`: 2 bytes, alignment: 1 bytes
print-type-size     variant `Some`: 2 bytes
print-type-size         field `.0`: 2 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<u8>`: 2 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Some`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `Enum4<(), (), bool, ()>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `Three`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `One`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Two`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Four`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size type: `MyOption<bool>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `Some`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<std::cmp::Ordering>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `Some`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `std::cmp::Ordering`: 1 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Less`: 0 bytes
print-type-size     variant `Equal`: 0 bytes
print-type-size     variant `Greater`: 0 bytes
stderr: none


---- [ui] src/test/ui/traits/issue-77982.rs stdout ----
---- [ui] src/test/ui/traits/issue-77982.rs stdout ----
diff of stderr:

46    |
47    = note: multiple `impl`s satisfying `u32: From<_>` found in the following crates: `core`, `std`:
48            - impl From<Ipv4Addr> for u32;
-            - impl From<NonZeroU32> for u32;
50            - impl From<bool> for u32;
51            - impl From<char> for u32;
+            - impl From<u16> for u32;
52            and 3 more
53 help: try using a fully qualified path to specify the expected types


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-77982.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-77982.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^ cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: Borrow<_>` found in the following crates: `alloc`, `core`:
           - impl Borrow<str> for String;
           - impl<T> Borrow<T> for T
             where T: ?Sized;
note: required by a bound in `HashMap::<K, V, S>::get`
   |
   |
LL |         K: Borrow<Q>,
   |            ^^^^^^^^^ required by this bound in `HashMap::<K, V, S>::get`
help: consider specifying the type argument in the function call
   |
LL |     opts.get::<Q>(opt.as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^     ------ type must be known at this point
   |          |
   |          cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: AsRef<_>` found in the following crates: `alloc`, `std`:
           - impl AsRef<OsStr> for String;
           - impl AsRef<Path> for String;
           - impl AsRef<[u8]> for String;
           - impl AsRef<str> for String;
help: consider specifying the generic argument
   |
LL |     opts.get::<Q>(opt.as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:13:59
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |                                            |
   |                                            type must be known at this point
   |
   |
   = note: multiple `impl`s satisfying `u32: From<_>` found in the following crates: `core`, `std`:
           - impl From<Ipv4Addr> for u32;
           - impl From<bool> for u32;
           - impl From<char> for u32;
           - impl From<u16> for u32;
           and 3 more
help: try using a fully qualified path to specify the expected types
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(<u32 as Into<T>>::into(0u32))).collect();

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:36:9
   |
   |
LL |     let _ = ().foo(); //~ ERROR type annotations needed
   |         ^      --- type must be known at this point
   |
note: multiple `impl`s satisfying `(): Foo<'_, _>` found
   |
   |
LL | impl Foo<'static, u32> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Foo<'a, i16> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider giving this pattern a type, where the type for type parameter `T` is specified
   |
LL |     let _: Box<T> = ().foo(); //~ ERROR type annotations needed

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:40:9
   |
   |
LL |     let _ = (&()).bar(); //~ ERROR type annotations needed
   |         ^         --- type must be known at this point
   |
note: multiple `impl`s satisfying `&(): Bar<'_, _>` found
   |
   |
LL | impl<'a> Bar<'static, u32> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Bar<'a, i16> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider giving this pattern a type, where the type for type parameter `T` is specified
   |
LL |     let _: Box<T> = (&()).bar(); //~ ERROR type annotations needed

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0283`.
For more information about this error, try `rustc --explain E0283`.
------------------------------------------


---- [ui] src/test/ui/try-trait/bad-interconversion.rs stdout ----
diff of stderr:

16              <f64 as From<i16>>
17              <f64 as From<i32>>
18              <f64 as From<i8>>
+            and 55 others
+            and 55 others
20    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, i32>>` for `Result<u64, u8>`
21 
22 error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/bad-interconversion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-trait/bad-interconversion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-trait/bad-interconversion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `?` couldn't convert the error to `u8`
   |
   |
LL | fn result_to_result() -> Result<u64, u8> {
   |                          --------------- expected `u8` because of this
LL |     Ok(Err(123_i32)?)
   |                    ^ the trait `From<i32>` is not implemented for `u8`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `From<T>`:
             <f32 as From<i16>>
             <f32 as From<i8>>
             <f32 as From<u16>>
             <f32 as From<u8>>
             <f64 as From<f32>>
             <f64 as From<i16>>
             <f64 as From<i32>>
             <f64 as From<i8>>
           and 55 others
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, i32>>` for `Result<u64, u8>`

error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
   |
   |
LL | / fn option_to_result() -> Result<u64, String> {
LL | |     Some(3)?;
   | |            ^ use `.ok_or(...)?` to provide an error compatible with `Result<u64, String>`
LL | |     //~^ ERROR the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
LL | |     Ok(10)
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `Result<u64, String>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Result<T, F> as FromResidual<Result<Infallible, E>>>
             <Result<T, F> as FromResidual<Yeet<E>>>

error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
   |
   |
LL | / fn control_flow_to_result() -> Result<u64, String> {
LL | |     Ok(ControlFlow::Break(123)?)
   | |                               ^ this `?` produces `ControlFlow<{integer}, Infallible>`, which is incompatible with `Result<u64, String>`
LL | |     //~^ ERROR the `?` operator can only be used on `Result`s in a function that returns `Result`
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Result<u64, String>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Result<T, F> as FromResidual<Result<Infallible, E>>>
             <Result<T, F> as FromResidual<Yeet<E>>>

error[E0277]: the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
   |
LL | / fn result_to_option() -> Option<u16> {
LL | / fn result_to_option() -> Option<u16> {
LL | |     Some(Err("hello")?)
   | |                      ^ use `.ok()?` if you want to discard the `Result<Infallible, &str>` error information
LL | |     //~^ ERROR the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `Option<u16>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Option<T> as FromResidual<Yeet<()>>>
             <Option<T> as FromResidual>

error[E0277]: the `?` operator can only be used on `Option`s in a function that returns `Option`
   |
LL | / fn control_flow_to_option() -> Option<u64> {
LL | / fn control_flow_to_option() -> Option<u64> {
LL | |     Some(ControlFlow::Break(123)?)
   | |                                 ^ this `?` produces `ControlFlow<{integer}, Infallible>`, which is incompatible with `Option<u64>`
LL | |     //~^ ERROR the `?` operator can only be used on `Option`s in a function that returns `Option`
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Option<u64>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Option<T> as FromResidual<Yeet<()>>>
             <Option<T> as FromResidual>

error[E0277]: the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
   |
   |
LL | / fn result_to_control_flow() -> ControlFlow<String> {
LL | |     ControlFlow::Continue(Err("hello")?)
   | |                                       ^ this `?` produces `Result<Infallible, &str>`, which is incompatible with `ControlFlow<String>`
LL | |     //~^ ERROR the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `ControlFlow<String>`
   = help: the trait `FromResidual` is implemented for `ControlFlow<B, C>`

error[E0277]: the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
