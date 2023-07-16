plain
.................................................................................................... 6200/12172
.......i.......................................................................i.................... 6300/12172
.................................................................................................... 6400/12172
.........................................ii.ii.......i...i.......................................... 6500/12172
.............F.F..F......................................................................i....i..... 6600/12172
.................................................................................................... 6800/12172
....i............................................................................................... 6900/12172
.....................i.............................................................................. 7000/12172
........................................ii...................................................i...... 7100/12172
---
diff of 64bit.stderr:

52   --> $DIR/ub-nonnull.rs:41:1
53    |
54 LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range AllocationRange { start: 10, end: 30 }
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range WrappingRange { start: 10, end: 30 }
56    |
57    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
58    = note: the raw bytes of the constant (size: 4, align: 4) {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:12:1
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:19:30
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:19:30
   |
LL |     let out_of_bounds_ptr = &ptr[255]; //~ ERROR evaluation of constant value failed
   |                              ^^^^^^^^ dereferencing pointer failed: alloc11 has size 1, so pointer to 256 bytes starting at offset 0 is out-of-bounds
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:23:1
   |
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:25:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:25:1
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:33:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:33:1
   |
LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               __                                              │ ░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:41:1
   |
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range WrappingRange { start: 10, end: 30 }
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               2a 00 00 00                                     │ *...

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:47:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: aborting due to 7 previous errors

---

15                 I32,
16                 false,
17             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
19                 start: 0,
20                 end: 0,

94                     I32,
95                     false,
96                 ),
96                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
98                     start: 0,
99                     end: 0,

144                 I32,
145                 true,
146             ),
146             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
148                 start: 0,
149                 end: 4294967295,

154                 I32,
155                 true,
156             ),
156             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
158                 start: 0,
159                 end: 4294967295,

219                 I32,
220                 false,
221             ),
221             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
223                 start: 0,
224                 end: 1,

291                 I32,
292                 false,
293             ),
293             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
295                 start: 0,
296                 end: 1,

301                 I32,
302                 true,
303             ),
303             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
305                 start: 0,
306                 end: 4294967295,

317                     I32,
318                     false,
319                 ),
319                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
321                     start: 0,
322                     end: 1,

350                 I32,
351                 true,
352             ),
352             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
354                 start: 0,
355                 end: 4294967295,


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/debug/debug.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/debug/debug.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args layout/debug.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/debug/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: layout_of(E) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I32,
                false,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 0,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 0,
                    },
                    pref: Align {
                        pow2: 3,
                },
                size: Size {
                    raw: 4,
                },
---
                },
                variants: Single {
                    index: 1,
                },
                abi: Uninhabited,
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 2,
                    },
                    pref: Align {
                        pow2: 3,
                },
                size: Size {
                    raw: 12,
                },
                },
            },
        ],
    },
    abi: Aggregate {
        sized: true,
    },
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I32,
                    false,
                    false,
                ),
                valid_range: WrappingRange {
                    start: 0,
                    end: 0,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 3,
    },
    size: Size {
        raw: 12,
    },
    },
}
  --> /checkout/src/test/ui/layout/debug.rs:6:1
   |
LL | enum E { Foo, Bar(!, i32, i32) } //~ ERROR: layout_of


error: layout_of(S) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
            Size {
                raw: 0,
---
            value: Int(
                I32,
                true,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 4294967295,
        },
        Scalar {
            value: Int(
                I32,
                I32,
                true,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 4294967295,
        },
    ),
    ),
    largest_niche: None,
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 3,
    },
    size: Size {
        raw: 8,
    },
    },
}
  --> /checkout/src/test/ui/layout/debug.rs:9:1
   |
LL | struct S { f1: i32, f2: (), f3: i32 } //~ ERROR: layout_of


error: layout_of(U) = Layout {
    fields: Union(
    ),
    variants: Single {
        index: 0,
    },
    },
    abi: Aggregate {
        sized: true,
    },
    largest_niche: None,
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 3,
    },
    size: Size {
        raw: 8,
    },
    },
}
  --> /checkout/src/test/ui/layout/debug.rs:12:1
   |
LL | union U { f1: (i32, i32), f3: i32 } //~ ERROR: layout_of


error: layout_of(std::result::Result<i32, i32>) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I32,
                false,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 1,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [
                            raw: 4,
                        },
                    ],
                    memory_index: [
---
                },
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 2,
                    },
                    pref: Align {
                        pow2: 3,
                },
                size: Size {
                    raw: 8,
                },
---
                },
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 2,
                    },
                    pref: Align {
                        pow2: 3,
                },
                size: Size {
                    raw: 8,
                },
---
            value: Int(
                I32,
                false,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 1,
        },
        Scalar {
            value: Int(
                I32,
                I32,
                true,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 4294967295,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I32,
                    false,
                    false,
                ),
                valid_range: WrappingRange {
                    start: 0,
                    end: 1,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 3,
    },
    size: Size {
        raw: 8,
    },
    },
}
  --> /checkout/src/test/ui/layout/debug.rs:15:1
   |
LL | type Test = Result<i32, i32>; //~ ERROR: layout_of


error: layout_of(i32) = Layout {
    fields: Primitive,
    variants: Single {
        index: 0,
    abi: Scalar(
        Scalar {
            value: Int(
                I32,
                I32,
                true,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 4294967295,
        },
    ),
    ),
    largest_niche: None,
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 2,
    },
    size: Size {
        raw: 4,
    },
    },
}
  --> /checkout/src/test/ui/layout/debug.rs:18:1
   |
LL | type T = impl std::fmt::Debug; //~ ERROR: layout_of

error: aborting due to 5 previous errors


---

15                 I8,
16                 false,
17             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
19                 start: 0,
20                 end: 0,

55                 I8,
56                 false,
57             ),
57             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
59                 start: 0,
60                 end: 0,

71                     I8,
72                     false,
73                 ),
73                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
75                     start: 0,
76                     end: 0,

112                 I8,
113                 false,
114             ),
114             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
116                 start: 255,
117                 end: 255,

152                 I8,
153                 false,
154             ),
154             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
156                 start: 255,
157                 end: 255,

168                     I8,
169                     false,
170                 ),
170                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
172                     start: 255,
173                     end: 255,

209                 I16,
210                 false,
211             ),
211             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
213                 start: 256,
214                 end: 256,

249                 I16,
250                 false,
251             ),
251             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
253                 start: 256,
254                 end: 256,

265                     I16,
266                     false,
267                 ),
267                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
269                     start: 256,
270                     end: 256,

306                 I32,
307                 false,
308             ),
308             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
310                 start: 268435456,
311                 end: 268435456,

346                 I32,
347                 false,
348             ),
348             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
350                 start: 268435456,
351                 end: 268435456,

362                     I32,
363                     false,
364                 ),
364                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
366                     start: 268435456,
367                     end: 268435456,

403                 I32,
404                 true,
405             ),
405             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
407                 start: 2164260864,
408                 end: 2164260864,

443                 I32,
444                 true,
445             ),
445             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
447                 start: 2164260864,
448                 end: 2164260864,

459                     I32,
460                     true,
461                 ),
461                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
463                     start: 2164260864,
464                     end: 2164260864,


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/hexagon-enum/hexagon-enum.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/hexagon-enum/hexagon-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args layout/hexagon-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/hexagon-enum.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/hexagon-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "hexagon-unknown-linux-musl" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/hexagon-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: layout_of(A) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I8,
                false,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 0,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 0,
                    },
                    pref: Align {
                        pow2: 0,
                },
                size: Size {
                    raw: 1,
                },
---
            value: Int(
                I8,
                false,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 0,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I8,
                    false,
                    false,
                ),
                valid_range: WrappingRange {
                    start: 0,
                    end: 0,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 0,
        },
        pref: Align {
            pow2: 0,
    },
    size: Size {
        raw: 1,
    },
    },
}
  --> /checkout/src/test/ui/layout/hexagon-enum.rs:16:1
   |
LL | enum A { Apple } //~ ERROR: layout_of


error: layout_of(B) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I8,
                false,
            ),
            valid_range: WrappingRange {
                start: 255,
                end: 255,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 0,
                    },
                    pref: Align {
                        pow2: 0,
                },
                size: Size {
                    raw: 1,
                },
---
            value: Int(
                I8,
                false,
            ),
            valid_range: WrappingRange {
                start: 255,
                end: 255,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I8,
                    false,
                    false,
                ),
                valid_range: WrappingRange {
                    start: 255,
                    end: 255,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 0,
        },
        pref: Align {
            pow2: 0,
    },
    size: Size {
        raw: 1,
    },
    },
}
  --> /checkout/src/test/ui/layout/hexagon-enum.rs:20:1
   |
LL | enum B { Banana = 255, } //~ ERROR: layout_of


error: layout_of(C) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I16,
                false,
            ),
            valid_range: WrappingRange {
                start: 256,
                end: 256,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 1,
                    },
                    pref: Align {
                        pow2: 1,
                },
                size: Size {
                    raw: 2,
                },
---
            value: Int(
                I16,
                false,
            ),
            valid_range: WrappingRange {
                start: 256,
                end: 256,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I16,
                    false,
                    false,
                ),
                valid_range: WrappingRange {
                    start: 256,
                    end: 256,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 1,
        },
        pref: Align {
            pow2: 1,
    },
    size: Size {
        raw: 2,
    },
    },
}
  --> /checkout/src/test/ui/layout/hexagon-enum.rs:24:1
   |
LL | enum C { Chaenomeles = 256, } //~ ERROR: layout_of


error: layout_of(P) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I32,
                false,
            ),
            valid_range: WrappingRange {
                start: 268435456,
                end: 268435456,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 2,
                    },
                    pref: Align {
                        pow2: 2,
                },
                size: Size {
                    raw: 4,
                },
---
            value: Int(
                I32,
                false,
            ),
            valid_range: WrappingRange {
                start: 268435456,
                end: 268435456,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I32,
                    false,
                    false,
                ),
                valid_range: WrappingRange {
                    start: 268435456,
                    end: 268435456,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 2,
    },
    size: Size {
        raw: 4,
    },
    },
}
  --> /checkout/src/test/ui/layout/hexagon-enum.rs:28:1
   |
LL | enum P { Peach = 0x1000_0000isize, } //~ ERROR: layout_of


error: layout_of(T) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I32,
                true,
            ),
            valid_range: WrappingRange {
                start: 2164260864,
                end: 2164260864,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 2,
                    },
                    pref: Align {
                        pow2: 2,
                },
                size: Size {
                    raw: 4,
                },
---
            value: Int(
                I32,
                true,
            ),
            valid_range: WrappingRange {
                start: 2164260864,
                end: 2164260864,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I32,
                    true,
                    true,
                ),
                valid_range: WrappingRange {
                    start: 2164260864,
                    end: 2164260864,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 2,
    },
    size: Size {
        raw: 4,
    },
    },
}
  --> /checkout/src/test/ui/layout/hexagon-enum.rs:34:1
   |
LL | enum T { Tangerine = TANGERINE as isize } //~ ERROR: layout_of

error: aborting due to 5 previous errors


---

15                 I8,
16                 false,
17             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
19                 start: 0,
20                 end: 0,

55                 I8,
56                 false,
57             ),
57             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
59                 start: 0,
60                 end: 0,

71                     I8,
72                     false,
73                 ),
73                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
75                     start: 0,
76                     end: 0,

112                 I8,
113                 false,
114             ),
114             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
116                 start: 255,
117                 end: 255,

152                 I8,
153                 false,
154             ),
154             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
156                 start: 255,
157                 end: 255,

168                     I8,
169                     false,
170                 ),
170                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
172                     start: 255,
173                     end: 255,

209                 I16,
210                 false,
211             ),
211             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
213                 start: 256,
214                 end: 256,

249                 I16,
250                 false,
251             ),
251             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
253                 start: 256,
254                 end: 256,

265                     I16,
266                     false,
267                 ),
267                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
269                     start: 256,
270                     end: 256,

306                 I32,
307                 false,
308             ),
308             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
310                 start: 268435456,
311                 end: 268435456,

346                 I32,
347                 false,
348             ),
348             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
350                 start: 268435456,
351                 end: 268435456,

362                     I32,
363                     false,
364                 ),
364                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
366                     start: 268435456,
367                     end: 268435456,

403                 I32,
404                 true,
405             ),
405             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
407                 start: 2164260864,
408                 end: 2164260864,

443                 I32,
444                 true,
445             ),
445             ),
-             valid_range: AllocationRange {
+             valid_range: WrappingRange {
447                 start: 2164260864,
448                 end: 2164260864,

459                     I32,
460                     true,
461                 ),
461                 ),
-                 valid_range: AllocationRange {
+                 valid_range: WrappingRange {
463                     start: 2164260864,
464                     end: 2164260864,


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/thumb-enum/thumb-enum.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/thumb-enum/thumb-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args layout/thumb-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/thumb-enum.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/thumb-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabihf" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/thumb-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: layout_of(A) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I8,
                false,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 0,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 0,
                    },
                    pref: Align {
                        pow2: 2,
                },
                size: Size {
                    raw: 1,
                },
---
            value: Int(
                I8,
                false,
            ),
            valid_range: WrappingRange {
                start: 0,
                end: 0,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I8,
                    false,
                    false,
                ),
                valid_range: WrappingRange {
                    start: 0,
                    end: 0,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 0,
        },
        pref: Align {
            pow2: 2,
    },
    size: Size {
        raw: 1,
    },
    },
}
  --> /checkout/src/test/ui/layout/thumb-enum.rs:16:1
   |
LL | enum A { Apple } //~ ERROR: layout_of


error: layout_of(B) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I8,
                false,
            ),
            valid_range: WrappingRange {
                start: 255,
                end: 255,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 0,
                    },
                    pref: Align {
                        pow2: 2,
                },
                size: Size {
                    raw: 1,
                },
---
            value: Int(
                I8,
                false,
            ),
            valid_range: WrappingRange {
                start: 255,
                end: 255,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I8,
                    false,
                    false,
                ),
                valid_range: WrappingRange {
                    start: 255,
                    end: 255,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 0,
        },
        pref: Align {
            pow2: 2,
    },
    size: Size {
        raw: 1,
    },
    },
}
  --> /checkout/src/test/ui/layout/thumb-enum.rs:20:1
   |
LL | enum B { Banana = 255, } //~ ERROR: layout_of


error: layout_of(C) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I16,
                false,
            ),
            valid_range: WrappingRange {
                start: 256,
                end: 256,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 1,
                    },
                    pref: Align {
                        pow2: 2,
                },
                size: Size {
                    raw: 2,
                },
---
            value: Int(
                I16,
                false,
            ),
            valid_range: WrappingRange {
                start: 256,
                end: 256,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I16,
                    false,
                    false,
                ),
                valid_range: WrappingRange {
                    start: 256,
                    end: 256,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 1,
        },
        pref: Align {
            pow2: 2,
    },
    size: Size {
        raw: 2,
    },
    },
}
  --> /checkout/src/test/ui/layout/thumb-enum.rs:24:1
   |
LL | enum C { Chaenomeles = 256, } //~ ERROR: layout_of


error: layout_of(P) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I32,
                false,
            ),
            valid_range: WrappingRange {
                start: 268435456,
                end: 268435456,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 2,
                    },
                    pref: Align {
                        pow2: 2,
                },
                size: Size {
                    raw: 4,
                },
---
            value: Int(
                I32,
                false,
            ),
            valid_range: WrappingRange {
                start: 268435456,
                end: 268435456,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I32,
                    false,
                    false,
                ),
                valid_range: WrappingRange {
                    start: 268435456,
                    end: 268435456,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 2,
    },
    size: Size {
        raw: 4,
    },
    },
}
  --> /checkout/src/test/ui/layout/thumb-enum.rs:28:1
   |
LL | enum P { Peach = 0x1000_0000isize, } //~ ERROR: layout_of


error: layout_of(T) = Layout {
    fields: Arbitrary {
        offsets: [
                raw: 0,
            },
        ],
        memory_index: [
---
            value: Int(
                I32,
                true,
            ),
            valid_range: WrappingRange {
                start: 2164260864,
                end: 2164260864,
        },
        tag_encoding: Direct,
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [],
                    memory_index: [],
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 2,
                    },
                    pref: Align {
                        pow2: 2,
                },
                size: Size {
                    raw: 4,
                },
---
            value: Int(
                I32,
                true,
            ),
            valid_range: WrappingRange {
                start: 2164260864,
                end: 2164260864,
        },
    ),
    ),
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            scalar: Scalar {
                value: Int(
                    I32,
                    true,
                    true,
                ),
                valid_range: WrappingRange {
                    start: 2164260864,
                    end: 2164260864,
            },
        },
    ),
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 2,
    },
    size: Size {
        raw: 4,
    },
    },
}
  --> /checkout/src/test/ui/layout/thumb-enum.rs:34:1
   |
LL | enum T { Tangerine = TANGERINE as isize } //~ ERROR: layout_of

error: aborting due to 5 previous errors


---
test result: FAILED. 12066 passed; 4 failed; 102 ignored; 0 measured; 0 filtered out; finished in 132.96s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:16
