plain

---- [ui] src/test/ui/layout/zero-sized-array-enum-niche.rs stdout ----
diff of stderr:

1 error: layout_of(std::result::Result<[u32; 0], bool>) = Layout {
+            size: Size(4 bytes),
+            align: AbiAndPrefAlign {
+                abi: Align(4 bytes),
+                pref: $PREF_ALIGN,
+            abi: Aggregate {
+                sized: true,
+            },
2            fields: Arbitrary {
2            fields: Arbitrary {
3                offsets: [
4                    Size(0 bytes),

7                    0,
8                ],
9            },
+            largest_niche: Some(
+                Niche {
+                    offset: Size(0 bytes),
+                    value: Int(
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+                        false,
+                    ),
+                    valid_range: 0..=1,
+                },
---

19                tag_field: 0,
20                variants: [
21                    Layout {
+                        size: Size(4 bytes),
+                        align: AbiAndPrefAlign {
+                            abi: Align(4 bytes),
+                            pref: $PREF_ALIGN,
+                        abi: Aggregate {
+                            sized: true,
+                        },
22                        fields: Arbitrary {
---
-                        },
-                        largest_niche: None,
+                    },
+                    Layout {
+                        size: Size(2 bytes),
37                        align: AbiAndPrefAlign {
-                            abi: Align(4 bytes),
+                            abi: Align(1 bytes),
39                            pref: $PREF_ALIGN,
-                        size: Size(4 bytes),
-                    },
-                    Layout {
+                        abi: Aggregate {
---
-                        },
-                        abi: Aggregate {
-                            sized: true,
-                        },
58                        largest_niche: Some(
59                            Niche {
60                                offset: Size(1 bytes),
65                                valid_range: 0..=1,
66                            },
67                        ),
67                        ),
-                        align: AbiAndPrefAlign {
-                            abi: Align(1 bytes),
-                            pref: $PREF_ALIGN,
+                        variants: Single {
71                        },
-                        size: Size(2 bytes),
73                    },
74                ],
74                ],
75            },

-            abi: Aggregate {
-                sized: true,
-            },
-            largest_niche: Some(
-                Niche {
-                    offset: Size(0 bytes),
-                    value: Int(
-                        false,
-                    ),
-                    valid_range: 0..=1,
-                },
-                },
-            ),
-            align: AbiAndPrefAlign {
-                abi: Align(4 bytes),
-                pref: $PREF_ALIGN,
-            size: Size(4 bytes),
94        }
95   --> $DIR/zero-sized-array-enum-niche.rs:13:1
96    |
96    |

98    | ^^^^^^^^^^^^^^^^^^
99 
100 error: layout_of(MultipleAlignments) = Layout {
+            size: Size(4 bytes),
+            align: AbiAndPrefAlign {
+                abi: Align(4 bytes),
+                pref: $PREF_ALIGN,
+            abi: Aggregate {
+                sized: true,
+            },
101            fields: Arbitrary {
101            fields: Arbitrary {
102                offsets: [
103                    Size(0 bytes),

106                    0,
107                ],
108            },
+            largest_niche: Some(
+                Niche {
+                    offset: Size(0 bytes),
+                    value: Int(
+                        false,
+                    ),
+                    valid_range: 0..=2,
+                },
---

118                tag_field: 0,
119                variants: [
120                    Layout {
+                        size: Size(2 bytes),
+                        align: AbiAndPrefAlign {
+                            abi: Align(2 bytes),
+                            pref: $PREF_ALIGN,
+                        abi: Aggregate {
+                            sized: true,
+                        },
121                        fields: Arbitrary {
---
-                        },
-                        largest_niche: None,
+                    },
+                    Layout {
+                        size: Size(4 bytes),
136                        align: AbiAndPrefAlign {
-                            abi: Align(2 bytes),
+                            abi: Align(4 bytes),
138                            pref: $PREF_ALIGN,
-                        size: Size(2 bytes),
-                    },
-                    Layout {
+                        abi: Aggregate {
---
-                        },
-                        largest_niche: None,
+                    },
+                    Layout {
+                        size: Size(2 bytes),
158                        align: AbiAndPrefAlign {
-                            abi: Align(4 bytes),
+                            abi: Align(1 bytes),
160                            pref: $PREF_ALIGN,
-                        size: Size(4 bytes),
-                    },
-                    Layout {
+                        abi: Aggregate {
---
-                        },
-                        abi: Aggregate {
-                            sized: true,
-                        },
179                        largest_niche: Some(
180                            Niche {
181                                offset: Size(1 bytes),
186                                valid_range: 0..=1,
187                            },
188                        ),
188                        ),
-                        align: AbiAndPrefAlign {
-                            abi: Align(1 bytes),
-                            pref: $PREF_ALIGN,
+                        variants: Single {
192                        },
-                        size: Size(2 bytes),
194                    },
195                ],
195                ],
196            },

-            abi: Aggregate {
-                sized: true,
-            },
-            largest_niche: Some(
-                Niche {
-                    offset: Size(0 bytes),
-                    value: Int(
-                        false,
-                    ),
-                    valid_range: 0..=2,
-                },
-                },
-            ),
-            align: AbiAndPrefAlign {
-                abi: Align(4 bytes),
-                pref: $PREF_ALIGN,
-            size: Size(4 bytes),
215        }
216   --> $DIR/zero-sized-array-enum-niche.rs:21:1
217    |
217    |

219    | ^^^^^^^^^^^^^^^^^^^^^^^
220 
221 error: layout_of(std::result::Result<[u32; 0], Packed<std::num::NonZeroU16>>) = Layout {
+            size: Size(4 bytes),
+            align: AbiAndPrefAlign {
+                abi: Align(4 bytes),
+                pref: $PREF_ALIGN,
+            abi: Aggregate {
+                sized: true,
+            },
222            fields: Arbitrary {
222            fields: Arbitrary {
223                offsets: [
224                    Size(0 bytes),

227                    0,
228                ],
229            },
+            largest_niche: Some(
+                Niche {
+                    offset: Size(0 bytes),
+                    value: Int(
+                        false,
+                    ),
+                    valid_range: 0..=1,
+                },
---

239                tag_field: 0,
240                variants: [
241                    Layout {
+                        size: Size(4 bytes),
+                        align: AbiAndPrefAlign {
+                            abi: Align(4 bytes),
+                            pref: $PREF_ALIGN,
+                        abi: Aggregate {
+                            sized: true,
+                        },
242                        fields: Arbitrary {
---
-                        },
-                        largest_niche: None,
+                    },
+                    Layout {
+                        size: Size(3 bytes),
257                        align: AbiAndPrefAlign {
-                            abi: Align(4 bytes),
+                            abi: Align(1 bytes),
259                            pref: $PREF_ALIGN,
-                        size: Size(4 bytes),
-                    },
-                    Layout {
+                        abi: Aggregate {
---
-                        },
-                        abi: Aggregate {
-                            sized: true,
-                        },
278                        largest_niche: Some(
279                            Niche {
280                                offset: Size(1 bytes),
285                                valid_range: 1..=65535,
286                            },
287                        ),
287                        ),
-                        align: AbiAndPrefAlign {
-                            abi: Align(1 bytes),
-                            pref: $PREF_ALIGN,
+                        variants: Single {
291                        },
-                        size: Size(3 bytes),
293                    },
294                ],
294                ],
295            },

-            abi: Aggregate {
-                sized: true,
-            },
-            largest_niche: Some(
-                Niche {
-                    offset: Size(0 bytes),
-                    value: Int(
-                        false,
-                    ),
-                    valid_range: 0..=1,
-                },
-                },
-            ),
-            align: AbiAndPrefAlign {
-                abi: Align(4 bytes),
-                pref: $PREF_ALIGN,
-            size: Size(4 bytes),
314        }
315   --> $DIR/zero-sized-array-enum-niche.rs:37:1
316    |
316    |

318    | ^^^^^^^^^^^^^^^^^^^^^^^
319 
320 error: layout_of(std::result::Result<[u32; 0], Packed<U16IsZero>>) = Layout {
+            size: Size(4 bytes),
+            align: AbiAndPrefAlign {
+                abi: Align(4 bytes),
+                pref: $PREF_ALIGN,
+            abi: Aggregate {
+                sized: true,
+            },
321            fields: Arbitrary {
321            fields: Arbitrary {
322                offsets: [
323                    Size(0 bytes),

326                    0,
327                ],
328            },
+            largest_niche: Some(
+                Niche {
+                    offset: Size(0 bytes),
+                    value: Int(
+                        false,
+                    ),
+                    valid_range: 0..=1,
+                },
---

342                tag_field: 0,
343                variants: [
344                    Layout {
+                        size: Size(0 bytes),
+                        align: AbiAndPrefAlign {
+                            abi: Align(4 bytes),
+                            pref: $PREF_ALIGN,
+                        abi: Aggregate {
+                            sized: true,
+                        },
345                        fields: Arbitrary {
---
-                        },
-                        largest_niche: None,
+                    },
+                    Layout {
+                        size: Size(2 bytes),
360                        align: AbiAndPrefAlign {
-                            abi: Align(4 bytes),
+                            abi: Align(1 bytes),
362                            pref: $PREF_ALIGN,
-                        size: Size(0 bytes),
-                    },
-                    Layout {
+                        abi: Aggregate {
---
-                        },
-                        abi: Aggregate {
-                            sized: true,
-                        },
381                        largest_niche: Some(
382                            Niche {
383                                offset: Size(0 bytes),
388                                valid_range: 0..=0,
389                            },
390                        ),
390                        ),
-                        align: AbiAndPrefAlign {
-                            abi: Align(1 bytes),
-                            pref: $PREF_ALIGN,
+                        variants: Single {
394                        },
-                        size: Size(2 bytes),
396                    },
397                ],
397                ],
398            },

-            abi: Aggregate {
-                sized: true,
-            },
-            largest_niche: Some(
-                Niche {
-                    offset: Size(0 bytes),
-                    value: Int(
-                        false,
-                    ),
-                    valid_range: 0..=1,
-                },
-                },
-            ),
-            align: AbiAndPrefAlign {
-                abi: Align(4 bytes),
-                pref: $PREF_ALIGN,
-            size: Size(4 bytes),
417        }
418   --> $DIR/zero-sized-array-enum-niche.rs:44:1
419    |
---
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


error: layout_of(std::result::Result<[u32; 0], Packed<std::num::NonZeroU16>>) = Layout {
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

