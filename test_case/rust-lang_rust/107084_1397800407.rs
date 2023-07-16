plain

---- [ui] tests/ui/layout/zero-sized-array-enum-niche.rs stdout ----
diff of stderr:

- error: layout_of(Adt(std::result::Result, [Array(Uint(U32), Const { ty: Uint(Usize), kind: Value(Leaf(0x0000000000000000)) }), Bool])) = Layout {
+ error: layout_of(Adt(std::result::Result, [Array(Uint(U32), Const { ty: Uint(Usize), kind: Value(Leaf(0x00000000)) }), Bool])) = Layout {
2            size: Size(4 bytes),
3            align: AbiAndPrefAlign {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
4                abi: Align(4 bytes),

218 LL | enum MultipleAlignments {
220 
220 
- error: layout_of(Adt(std::result::Result, [Array(Uint(U32), Const { ty: Uint(Usize), kind: Value(Leaf(0x0000000000000000)) }), Adt(Packed, [Adt(std::num::NonZeroU16, [])])])) = Layout {
+ error: layout_of(Adt(std::result::Result, [Array(Uint(U32), Const { ty: Uint(Usize), kind: Value(Leaf(0x00000000)) }), Adt(Packed, [Adt(std::num::NonZeroU16, [])])])) = Layout {
222            size: Size(4 bytes),
223            align: AbiAndPrefAlign {
224                abi: Align(4 bytes),

317 LL | type NicheLosesToTagged = Result<[u32; 0], Packed<std::num::NonZeroU16>>;
319 
319 
- error: layout_of(Adt(std::result::Result, [Array(Uint(U32), Const { ty: Uint(Usize), kind: Value(Leaf(0x0000000000000000)) }), Adt(Packed, [Adt(U16IsZero, [])])])) = Layout {
+ error: layout_of(Adt(std::result::Result, [Array(Uint(U32), Const { ty: Uint(Usize), kind: Value(Leaf(0x00000000)) }), Adt(Packed, [Adt(U16IsZero, [])])])) = Layout {
321            size: Size(4 bytes),
322            align: AbiAndPrefAlign {
323                abi: Align(4 bytes),

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/zero-sized-array-enum-niche/zero-sized-array-enum-niche.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args layout/zero-sized-array-enum-niche.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/layout/zero-sized-array-enum-niche.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/zero-sized-array-enum-niche" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/zero-sized-array-enum-niche/auxiliary"
stdout: none
--- stderr -------------------------------
error: layout_of(Adt(std::result::Result, [Array(Uint(U32), Const { ty: Uint(Usize), kind: Value(Leaf(0x00000000)) }), Bool])) = Layout {
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
                       I8,
                       false,
                   ),
                   valid_range: 0..=1,
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
                                   I8,
                                   false,
                               ),
                               valid_range: 0..=1,
---
           },
       }
  --> /checkout/tests/ui/layout/zero-sized-array-enum-niche.rs:13:1
   |
LL | type AlignedResult = Result<[u32; 0], bool>; //~ ERROR: layout_of


error: layout_of(Adt(MultipleAlignments, [])) = Layout {
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
                       I8,
                       false,
                   ),
                   valid_range: 0..=2,
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
                                   I8,
                                   false,
                               ),
                               valid_range: 0..=1,
---
           },
       }
  --> /checkout/tests/ui/layout/zero-sized-array-enum-niche.rs:21:1
   |
LL | enum MultipleAlignments { //~ ERROR: layout_of


error: layout_of(Adt(std::result::Result, [Array(Uint(U32), Const { ty: Uint(Usize), kind: Value(Leaf(0x00000000)) }), Adt(Packed, [Adt(std::num::NonZeroU16, [])])])) = Layout {
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
                       I8,
                       false,
                   ),
                   valid_range: 0..=1,
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
                                   I16,
                                   false,
                               ),
                               valid_range: 1..=65535,
---
           },
       }
  --> /checkout/tests/ui/layout/zero-sized-array-enum-niche.rs:37:1
   |
LL | type NicheLosesToTagged = Result<[u32; 0], Packed<std::num::NonZeroU16>>; //~ ERROR: layout_of


error: layout_of(Adt(std::result::Result, [Array(Uint(U32), Const { ty: Uint(Usize), kind: Value(Leaf(0x00000000)) }), Adt(Packed, [Adt(U16IsZero, [])])])) = Layout {
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
                       I16,
                       false,
                   ),
                   valid_range: 0..=1,
---
                       false,
                   ),
                   valid_range: 0..=1,
               },
               tag_encoding: Niche {
                   untagged_variant: 1,
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
                                   I16,
                                   false,
                               ),
                               valid_range: 0..=0,
---
           },
       }
  --> /checkout/tests/ui/layout/zero-sized-array-enum-niche.rs:44:1
   |
LL | type NicheWinsOverTagged = Result<[u32; 0], Packed<U16IsZero>>; //~ ERROR: layout_of

error: aborting due to 4 previous errors
------------------------------------------

