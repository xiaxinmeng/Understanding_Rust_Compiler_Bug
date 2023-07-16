plain
test [ui] src/test/ui/layout/homogeneous-aggr-zero-sized-c-struct.rs ... ok
test [ui] src/test/ui/iterators/iter-range.rs ... ok
test [ui] src/test/ui/layout/homogeneous-aggr-zero-sized-repr-rust.rs ... ok
test [ui] src/test/ui/layout/issue-84108.rs ... ok
test [ui] src/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit.rs ... FAILED
test [ui] src/test/ui/iterators/iter-step-overflow-ndebug.rs ... ok
test [ui] src/test/ui/layout/zero-sized-array-union.rs ... ok
test [ui] src/test/ui/iterators/iter-sum-overflow-debug.rs ... ok
test [ui] src/test/ui/iterators/iter-sum-overflow-ndebug.rs ... ok
---
test [ui] src/test/ui/threads-sendsync/mpsc_stress.rs ... ok

failures:

---- [ui] src/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit.rs stdout ----

43                                pow2: 0,
44                            },
44                            },
45                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=arm-unknown-linux-gnueabihf
48                        },
49                        size: Size {

67                                pow2: 0,
67                                pow2: 0,
68                            },
69                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
72                        },
73                        size: Size {

108                    pow2: 0,
108                    pow2: 0,
109                },
110                pref: Align {
-                    pow2: 3,
+                    pow2: 2,
113            },
114            size: Size {

168                                pow2: 0,
168                                pow2: 0,
169                            },
170                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
173                        },
174                        size: Size {

198                                pow2: 0,
198                                pow2: 0,
199                            },
200                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
203                        },
204                        size: Size {

240                    pow2: 0,
240                    pow2: 0,
241                },
242                pref: Align {
-                    pow2: 3,
+                    pow2: 2,
245            },
246            size: Size {

300                                pow2: 0,
300                                pow2: 0,
301                            },
302                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
305                        },
306                        size: Size {

330                                pow2: 0,
330                                pow2: 0,
331                            },
332                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
335                        },
336                        size: Size {

371                    pow2: 0,
371                    pow2: 0,
372                },
373                pref: Align {
-                    pow2: 3,
+                    pow2: 2,
376            },
377            size: Size {

463                                pow2: 0,
463                                pow2: 0,
464                            },
465                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
468                        },
469                        size: Size {

487                                pow2: 0,
487                                pow2: 0,
488                            },
489                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
492                        },
493                        size: Size {

511                                pow2: 0,
511                                pow2: 0,
512                            },
513                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
516                        },
517                        size: Size {

552                    pow2: 0,
552                    pow2: 0,
553                },
554                pref: Align {
-                    pow2: 3,
+                    pow2: 2,
557            },
558            size: Size {

645                                pow2: 0,
645                                pow2: 0,
646                            },
647                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
650                        },
651                        size: Size {

669                                pow2: 0,
669                                pow2: 0,
670                            },
671                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
674                        },
675                        size: Size {

693                                pow2: 0,
693                                pow2: 0,
694                            },
695                            pref: Align {
-                                pow2: 3,
+                                pow2: 2,
698                        },
699                        size: Size {

734                    pow2: 0,
734                    pow2: 0,
735                },
736                pref: Align {
-                    pow2: 3,
+                    pow2: 2,
739            },
740            size: Size {



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit/issue-96158-scalarpair-payload-might-be-uninit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args layout/issue-96158-scalarpair-payload-might-be-uninit.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit/auxiliary"
stdout: none
--- stderr -------------------------------
error: layout_of(MissingPayloadField) = Layout {
           fields: Arbitrary {
               offsets: [
                       raw: 0,
                   },
               ],
               memory_index: [
---
                       false,
                   ),
                   valid_range: 0..=1,
               },
               tag_encoding: Direct,
               tag_field: 0,
               variants: [
                       fields: Arbitrary {
                           offsets: [
                               Size {
                                   raw: 1,
---
                       },
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
                           raw: 2,
                       },
---
                       },
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
                   ),
                   valid_range: 0..=1,
               },
           ),
           align: AbiAndPrefAlign {
               abi: Align {
                   pow2: 0,
               },
               pref: Align {
                   pow2: 2,
           },
           size: Size {
               raw: 2,
           },
           },
       }
  --> /checkout/src/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit.rs:15:1
   |
LL | / pub enum MissingPayloadField { //~ ERROR: layout_of
LL | |     Some(u8),
LL | |     None
LL | | }


error: layout_of(CommonPayloadField) = Layout {
           fields: Arbitrary {
               offsets: [
                       raw: 0,
                   },
               ],
               memory_index: [
---
                       false,
                   ),
                   valid_range: 0..=1,
               },
               tag_encoding: Direct,
               tag_field: 0,
               variants: [
                       fields: Arbitrary {
                           offsets: [
                               Size {
                                   raw: 1,
---
                       },
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
                           raw: 2,
                       },
---
                       },
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
                           raw: 2,
                       },
---
                   ),
                   valid_range: 0..=1,
               },
           ),
           align: AbiAndPrefAlign {
               abi: Align {
                   pow2: 0,
               },
               pref: Align {
                   pow2: 2,
           },
           size: Size {
               raw: 2,
           },
           },
       }
  --> /checkout/src/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit.rs:24:1
   |
LL | / pub enum CommonPayloadField { //~ ERROR: layout_of
LL | |     A(u8),
LL | |     B(u8),
LL | | }


error: layout_of(CommonPayloadFieldIsMaybeUninit) = Layout {
           fields: Arbitrary {
               offsets: [
                       raw: 0,
                   },
               ],
               memory_index: [
---
                       false,
                   ),
                   valid_range: 0..=1,
               },
               tag_encoding: Direct,
               tag_field: 0,
               variants: [
                       fields: Arbitrary {
                           offsets: [
                               Size {
                                   raw: 1,
---
                       },
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
                           raw: 2,
                       },
---
                       },
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
                           raw: 2,
                       },
---
                   ),
                   valid_range: 0..=1,
               },
           ),
           align: AbiAndPrefAlign {
               abi: Align {
                   pow2: 0,
               },
               pref: Align {
                   pow2: 2,
           },
           size: Size {
               raw: 2,
           },
           },
       }
  --> /checkout/src/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit.rs:32:1
   |
LL | / pub enum CommonPayloadFieldIsMaybeUninit { //~ ERROR: layout_of
LL | |     A(u8),
LL | |     B(MaybeUninit<u8>),
LL | | }


error: layout_of(NicheFirst) = Layout {
           fields: Arbitrary {
               offsets: [
                       raw: 0,
                   },
               ],
               memory_index: [
---
                       false,
                   ),
                   valid_range: 0..=4,
               },
               tag_encoding: Niche {
                   dataful_variant: 0,
                   niche_variants: 1..=2,
                   niche_start: 3,
               },
               tag_field: 0,
               variants: [
                       fields: Arbitrary {
                           offsets: [
                               Size {
                                   raw: 0,
---
                               ),
                               valid_range: 0..=2,
                           },
                       ),
                       align: AbiAndPrefAlign {
                           abi: Align {
                               pow2: 0,
                           },
                           pref: Align {
                               pow2: 2,
                       },
                       size: Size {
                           raw: 2,
                       },
---
                       },
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
                           raw: 0,
                       },
---
                       },
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
                           raw: 0,
                       },
---
                   ),
                   valid_range: 0..=4,
               },
           ),
           align: AbiAndPrefAlign {
               abi: Align {
                   pow2: 0,
               },
               pref: Align {
                   pow2: 2,
           },
           size: Size {
               raw: 2,
           },
           },
       }
  --> /checkout/src/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit.rs:40:1
   |
LL | / pub enum NicheFirst { //~ ERROR: layout_of
LL | |     A(HasNiche, u8),
LL | |     B,
LL | |     C
LL | | }


error: layout_of(NicheSecond) = Layout {
           fields: Arbitrary {
               offsets: [
                       raw: 1,
                   },
               ],
               memory_index: [
---
                       false,
                   ),
                   valid_range: 0..=4,
               },
               tag_encoding: Niche {
                   dataful_variant: 0,
                   niche_variants: 1..=2,
                   niche_start: 3,
               },
               tag_field: 0,
               variants: [
                       fields: Arbitrary {
                           offsets: [
                               Size {
                                   raw: 0,
---
                               ),
                               valid_range: 0..=2,
                           },
                       ),
                       align: AbiAndPrefAlign {
                           abi: Align {
                               pow2: 0,
                           },
                           pref: Align {
                               pow2: 2,
                       },
                       size: Size {
                           raw: 2,
                       },
---
                       },
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
                           raw: 0,
                       },
---
                       },
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
                           raw: 0,
                       },
---
                   ),
                   valid_range: 0..=4,
               },
           ),
           align: AbiAndPrefAlign {
               abi: Align {
                   pow2: 0,
               },
               pref: Align {
                   pow2: 2,
           },
           size: Size {
               raw: 2,
           },
           },
       }
  --> /checkout/src/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit.rs:49:1
   |
LL | / pub enum NicheSecond { //~ ERROR: layout_of
LL | |     A(u8, HasNiche),
LL | |     B,
LL | |     C,
LL | | }

error: aborting due to 5 previous errors
------------------------------------------




failures:
    [ui] src/test/ui/layout/issue-96158-scalarpair-payload-might-be-uninit.rs
test result: FAILED. 12780 passed; 1 failed; 171 ignored; 0 measured; 0 filtered out; finished in 599.42s

Build completed unsuccessfully in 0:23:11
