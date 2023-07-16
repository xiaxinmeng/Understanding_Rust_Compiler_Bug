plain

---- [ui] src/test/ui/fmt/ifmt-unimpl.rs stdout ----
diff of stderr:

7    = help: the following other types implement trait `UpperHex`:
8              &T
-              NonZeroI128
-              NonZeroI16
-              NonZeroI32
-              NonZeroI64
---
+              core::num::nonzero::NonZero<i128>
+              core::num::nonzero::NonZero<i16>
+              core::num::nonzero::NonZero<i32>
16            and 21 others
17    = note: required because of the requirements on the impl of `UpperHex` for `&str`
18 note: required by a bound in `ArgumentV1::<'a>::new_upper_hex`

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
             core::num::nonzero::NonZero<i128>
             core::num::nonzero::NonZero<i128>
             core::num::nonzero::NonZero<i16>
             core::num::nonzero::NonZero<i32>
           and 21 others
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
       }
  --> /checkout/src/test/ui/layout/zero-sized-array-enum-niche.rs:44:1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
