plain

---- [ui] src/test/ui/layout/issue-96185-overaligned-enum.rs stdout ----
diff of stderr:

1 error: layout_of(Aligned1) = Layout {
2            fields: Arbitrary {
3                offsets: [
-                        raw: 0,
-                    },
+                    Size(0 bytes),
7                ],
7                ],
8                memory_index: [
9                    0,

33                        },
34                        largest_niche: None,
35                        align: AbiAndPrefAlign {
-                            abi: Align {
-                                pow2: 3,
-                            },
-                            pref: $PREF_ALIGN,
+                            abi: Align(8 bytes),
+                            pref: Align(8 bytes),
-                        size: Size {
-                            raw: 8,
-                        },
+                        size: Size(8 bytes),
+                        size: Size(8 bytes),
44                    },
45                    Layout {
46                        fields: Arbitrary {

55                        },
56                        largest_niche: None,
57                        align: AbiAndPrefAlign {
-                            abi: Align {
-                                pow2: 3,
-                            },
-                            pref: $PREF_ALIGN,
+                            abi: Align(8 bytes),
+                            pref: Align(8 bytes),
-                        size: Size {
-                            raw: 8,
-                        },
+                        size: Size(8 bytes),
---
73                Niche {
-                    offset: Size {
-                        raw: 0,
-                    },
+                    offset: Size(0 bytes),
77                    value: Int(
79                        false,

82                },
83            ),
83            ),
84            align: AbiAndPrefAlign {
-                abi: Align {
-                    pow2: 3,
-                },
-                pref: $PREF_ALIGN,
+                abi: Align(8 bytes),
+                pref: Align(8 bytes),
-            size: Size {
-                raw: 8,
-            },
+            size: Size(8 bytes),
+            size: Size(8 bytes),
93        }
94   --> $DIR/issue-96185-overaligned-enum.rs:8:1
95    |

- LL | / pub enum Aligned1 {
- LL | |     Zero = 0,
- LL | |     One = 1,
- LL | | }
-    | |_^
+ LL | pub enum Aligned1 {
101 
101 
102 error: layout_of(Aligned2) = Layout {
103            fields: Arbitrary {
104                offsets: [
-                    Size {
-                        raw: 0,
-                    },
-                    },
+                    Size(0 bytes),
108                ],
109                memory_index: [
110                    0,

134                        },
135                        largest_niche: None,
136                        align: AbiAndPrefAlign {
-                            abi: Align {
-                                pow2: 0,
-                            },
-                            pref: $PREF_ALIGN,
+                            abi: Align(1 bytes),
+                            pref: Align(8 bytes),
-                        size: Size {
-                            raw: 1,
-                        },
+                        size: Size(1 bytes),
+                        size: Size(1 bytes),
145                    },
146                    Layout {
147                        fields: Arbitrary {

156                        },
157                        largest_niche: None,
158                        align: AbiAndPrefAlign {
-                            abi: Align {
-                                pow2: 0,
-                            },
-                            pref: $PREF_ALIGN,
+                            abi: Align(1 bytes),
+                            pref: Align(8 bytes),
-                        size: Size {
-                            raw: 1,
-                        },
+                        size: Size(1 bytes),
---
180                Niche {
-                    offset: Size {
-                        raw: 0,
-                    },
+                    offset: Size(0 bytes),
184                    value: Int(
186                        false,

189                },
190            ),
190            ),
191            align: AbiAndPrefAlign {
-                abi: Align {
-                    pow2: 0,
-                },
-                pref: $PREF_ALIGN,
+                abi: Align(1 bytes),
+                pref: Align(8 bytes),
-            size: Size {
-                raw: 1,
-            },
+            size: Size(1 bytes),
+            size: Size(1 bytes),
200        }
201   --> $DIR/issue-96185-overaligned-enum.rs:16:1
202    |

- LL | / pub enum Aligned2 {
- LL | |     Zero = 0,
- LL | |     One = 1,
- LL | | }
-    | |_^
+ LL | pub enum Aligned2 {
208 
209 error: aborting due to 2 previous errors
210 

---
To only update this specific test, also pass `--test-args layout/issue-96185-overaligned-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/issue-96185-overaligned-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/issue-96185-overaligned-enum" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/issue-96185-overaligned-enum/auxiliary"
stdout: none
--- stderr -------------------------------
error: layout_of(Aligned1) = Layout {
           fields: Arbitrary {
               offsets: [
                   Size(0 bytes),
               memory_index: [
                   0,
               ],
           },
---
                       false,
                   ),
                   valid_range: 0..=1,
               },
               tag_encoding: Direct,
               tag_field: 0,
               variants: [
                       fields: Arbitrary {
                           offsets: [],
                           memory_index: [],
                       },
                       },
                       variants: Single {
                           index: 0,
                       },
                       abi: Aggregate {
                           sized: true,
                       },
                       largest_niche: None,
                       align: AbiAndPrefAlign {
                           abi: Align(8 bytes),
                           pref: Align(8 bytes),
                       size: Size(8 bytes),
                   },
                   Layout {
                       fields: Arbitrary {
---
                       },
                       abi: Aggregate {
                           sized: true,
                       },
                       largest_niche: None,
                       align: AbiAndPrefAlign {
                           abi: Align(8 bytes),
                           pref: Align(8 bytes),
                       size: Size(8 bytes),
                   },
               ],
           },
           },
           abi: Aggregate {
               sized: true,
           },
           largest_niche: Some(
               Niche {
                   offset: Size(0 bytes),
                   value: Int(
                       false,
                   ),
                   valid_range: 0..=1,
               },
               },
           ),
           align: AbiAndPrefAlign {
               abi: Align(8 bytes),
               pref: Align(8 bytes),
           size: Size(8 bytes),
       }
  --> /checkout/src/test/ui/layout/issue-96185-overaligned-enum.rs:8:1
   |
   |
LL | pub enum Aligned1 { //~ ERROR: layout_of


error: layout_of(Aligned2) = Layout {
           fields: Arbitrary {
               offsets: [
                   Size(0 bytes),
               memory_index: [
                   0,
               ],
           },
---
                       false,
                   ),
                   valid_range: 0..=1,
               },
               tag_encoding: Direct,
               tag_field: 0,
               variants: [
                       fields: Arbitrary {
                           offsets: [],
                           memory_index: [],
                       },
                       },
                       variants: Single {
                           index: 0,
                       },
                       abi: Aggregate {
                           sized: true,
                       },
                       largest_niche: None,
                       align: AbiAndPrefAlign {
                           abi: Align(1 bytes),
                           pref: Align(8 bytes),
                       size: Size(1 bytes),
                   },
                   Layout {
                       fields: Arbitrary {
---
                       },
                       abi: Aggregate {
                           sized: true,
                       },
                       largest_niche: None,
                       align: AbiAndPrefAlign {
                           abi: Align(1 bytes),
                           pref: Align(8 bytes),
                       size: Size(1 bytes),
                   },
               ],
           },
---
                   ),
                   valid_range: 0..=1,
               },
           ),
           largest_niche: Some(
               Niche {
                   offset: Size(0 bytes),
                   value: Int(
                       false,
                   ),
                   valid_range: 0..=1,
               },
               },
           ),
           align: AbiAndPrefAlign {
               abi: Align(1 bytes),
               pref: Align(8 bytes),
           size: Size(1 bytes),
       }
  --> /checkout/src/test/ui/layout/issue-96185-overaligned-enum.rs:16:1
   |
   |
LL | pub enum Aligned2 { //~ ERROR: layout_of

error: aborting due to 2 previous errors
------------------------------------------

