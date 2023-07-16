plain

---- compile_test stdout ----
diff of stderr:

-error: Intel x86 assembly syntax used
-  --> $DIR/asm_syntax.rs:9:9
+error: cannot find macro `asm` in this scope
    |
    |
-LL |         asm!("");
-   |         ^^^^^^^^
+LL |         asm!("", options(nostack, att_syntax));
    |
    |
-   = note: `-D clippy::inline-asm-x86-intel-syntax` implied by `-D warnings`
-   = help: use AT&T x86 assembly syntax
+           std::arch::asm
+           core::arch::asm
 
 
-error: Intel x86 assembly syntax used
-  --> $DIR/asm_syntax.rs:10:9
+error: cannot find macro `asm` in this scope
    |
    |
+LL |         asm!("", options(att_syntax));
+   |
+   = note: consider importing one of these items:
+           std::arch::asm
+           core::arch::asm
+           core::arch::asm
+
+error: cannot find macro `asm` in this scope
+   |
+   |
+LL |         asm!("", options(nostack));
+   |
+   = note: consider importing one of these items:
+           std::arch::asm
+           core::arch::asm
+           core::arch::asm
+
+error: cannot find macro `asm` in this scope
+   |
+   |
 LL |         asm!("", options());
-   |         ^^^^^^^^^^^^^^^^^^^
    |
    |
-   = help: use AT&T x86 assembly syntax
+           std::arch::asm
+           core::arch::asm
 
 
-error: Intel x86 assembly syntax used
+error: cannot find macro `asm` in this scope
+   |
+   |
+LL |         asm!("");
+   |
+   = note: consider importing one of these items:
+           std::arch::asm
+           core::arch::asm
+           core::arch::asm
+
+error: cannot find macro `asm` in this scope
+   |
+   |
+LL |         asm!("", options(nostack, att_syntax));
+   |
+   = note: consider importing one of these items:
+           std::arch::asm
+           core::arch::asm
+           core::arch::asm
+
+error: cannot find macro `asm` in this scope
+   |
+   |
+LL |         asm!("", options(att_syntax));
+   |
+   = note: consider importing one of these items:
+           std::arch::asm
+           core::arch::asm
+           core::arch::asm
+
+error: cannot find macro `asm` in this scope
    |
    |
 LL |         asm!("", options(nostack));
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
-   = help: use AT&T x86 assembly syntax
+           std::arch::asm
+           core::arch::asm
 
 
-error: AT&T x86 assembly syntax used
-  --> $DIR/asm_syntax.rs:23:9
+error: cannot find macro `asm` in this scope
    |
    |
-LL |         asm!("", options(att_syntax));
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+LL |         asm!("", options());
    |
    |
-   = note: `-D clippy::inline-asm-x86-att-syntax` implied by `-D warnings`
-   = help: use Intel x86 assembly syntax
+           std::arch::asm
+           core::arch::asm
 
 
-error: AT&T x86 assembly syntax used
-  --> $DIR/asm_syntax.rs:24:9
+error: cannot find macro `asm` in this scope
    |
    |
-LL |         asm!("", options(nostack, att_syntax));
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+LL |         asm!("");
    |
    |
-   = help: use Intel x86 assembly syntax
+           std::arch::asm
+           core::arch::asm
 
-error: aborting due to 5 previous errors
---

error: 1 errors occurred comparing output.
status: exit status: 1
error: test failed, to rerun pass '--test compile-test'
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/asm_syntax.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/asm_syntax.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/asm_syntax.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/asm_syntax.rs","byte_start":555,"byte_end":558,"line_start":24,"line_end":24,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        asm!(\"\", options(nostack, att_syntax));","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/asm_syntax.rs:24:9\n   |\nLL |         asm!(\"\", options(nostack, att_syntax));\n   |         ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/asm_syntax.rs","byte_start":516,"byte_end":519,"line_start":23,"line_end":23,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        asm!(\"\", options(att_syntax));","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/asm_syntax.rs:23:9\n   |\nLL |         asm!(\"\", options(att_syntax));\n   |         ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/asm_syntax.rs","byte_start":480,"byte_end":483,"line_start":22,"line_end":22,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        asm!(\"\", options(nostack));","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/asm_syntax.rs:22:9\n   |\nLL |         asm!(\"\", options(nostack));\n   |         ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/asm_syntax.rs","byte_start":451,"byte_end":454,"line_start":21,"line_end":21,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        asm!(\"\", options());","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/asm_syntax.rs:21:9\n   |\nLL |         asm!(\"\", options());\n   |         ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/asm_syntax.rs","byte_start":433,"byte_end":436,"line_start":20,"line_end":20,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        asm!(\"\");","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/asm_syntax.rs:20:9\n   |\nLL |         asm!(\"\");\n   |         ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/asm_syntax.rs","byte_start":281,"byte_end":284,"line_start":13,"line_end":13,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        asm!(\"\", options(nostack, att_syntax));","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/asm_syntax.rs:13:9\n   |\nLL |         asm!(\"\", options(nostack, att_syntax));\n   |         ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/asm_syntax.rs","byte_start":242,"byte_end":245,"line_start":12,"line_end":12,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        asm!(\"\", options(att_syntax));","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/asm_syntax.rs:12:9\n   |\nLL |         asm!(\"\", options(att_syntax));\n   |         ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/asm_syntax.rs","byte_start":206,"byte_end":209,"line_start":11,"line_end":11,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        asm!(\"\", options(nostack));","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/asm_syntax.rs:11:9\n   |\nLL |         asm!(\"\", options(nostack));\n   |         ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/asm_syntax.rs","byte_start":177,"byte_end":180,"line_start":10,"line_end":10,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        asm!(\"\", options());","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/asm_syntax.rs:10:9\n   |\nLL |         asm!(\"\", options());\n   |         ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/asm_syntax.rs","byte_start":159,"byte_end":162,"line_start":9,"line_end":9,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        asm!(\"\");","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/asm_syntax.rs:9:9\n   |\nLL |         asm!(\"\");\n   |         ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: usage of `contains_key` followed by `insert` on a `HashMap`
-  --> $DIR/entry.rs:24:5
+error: cannot find macro `asm` in this scope
    |
    |
-LL | /     if !m.contains_key(&k) {
-LL | |         m.insert(k, v);
-LL | |     }
-   | |_____^ help: try this: `m.entry(k).or_insert(v);`
+LL |         unsafe { asm!("nop") }
    |
    |
-   = note: `-D clippy::map-entry` implied by `-D warnings`
+           std::arch::asm
+           core::arch::asm
 
 
-error: usage of `contains_key` followed by `insert` on a `HashMap`
-   |
-   |
-LL | /     if !m.contains_key(&k) {
-LL | |         if true {
-LL | |             m.insert(k, v);
-LL | |         } else {
-LL | |             m.insert(k, v2);
-LL | |         }
-LL | |     }
-   | |_____^
-   |
-   |
-   |
-LL ~     m.entry(k).or_insert_with(|| {
-LL +         if true {
-LL +             v
-LL +         } else {
-LL +             v2
-LL +         }
-
-
-error: usage of `contains_key` followed by `insert` on a `HashMap`
-   |
-   |
-LL | /     if !m.contains_key(&k) {
-LL | |         if true {
-LL | |             m.insert(k, v)
-LL | |         } else {
-LL | |             m.insert(k, v2)
-LL | |         };
-LL | |     }
-   | |_____^
-   |
-   |
-   |
-LL ~     m.entry(k).or_insert_with(|| {
-LL +         if true {
-LL +             v
-LL +         } else {
-LL +             v2
-LL +         }
-
-
-error: usage of `contains_key` followed by `insert` on a `HashMap`
-   |
-   |
-LL | /     if !m.contains_key(&k) {
-LL | |         if true {
-LL | |             m.insert(k, v);
-LL | |         } else {
-LL | |         }
-LL | |     }
-   | |_____^
-   |
-   |
-help: try this
-   |
-LL ~     if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {
-LL +         if true {
-LL +             e.insert(v);
-LL +         } else {
-LL +             e.insert(v2);
-LL +             return;
-
-
-error: usage of `contains_key` followed by `insert` on a `HashMap`
-   |
-   |
-LL | /     if !m.contains_key(&k) {
-LL | |         foo();
-LL | |         m.insert(k, v);
-LL | |     }
-   | |_____^
-   |
-   |
-   |
-LL ~     m.entry(k).or_insert_with(|| {
-LL +         foo();
-LL +         v
-LL +     });
-   |
-
-error: usage of `contains_key` followed by `insert` on a `HashMap`
-   |
-   |
-LL | /     if !m.contains_key(&k) {
-LL | |         match 0 {
-LL | |             1 if true => {
-LL | |                 m.insert(k, v);
-LL | |         };
-LL | |     }
-   | |_____^
-   |
-   |
-help: try this
-   |
-LL ~     m.entry(k).or_insert_with(|| {
-LL +         match 0 {
-LL +             1 if true => {
-LL +                 v
-LL +             },
-LL +             _ => {
-
-
-error: usage of `contains_key` followed by `insert` on a `HashMap`
-   |
-   |
-LL | /     if !m.contains_key(&k) {
-LL | |         match 0 {
-LL | |             0 => foo(),
-LL | |             _ => {
-LL | |         };
-LL | |     }
-   | |_____^
-   |
-   |
-help: try this
-   |
-LL ~     if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {
-LL +         match 0 {
-LL +             0 => foo(),
-LL +             _ => {
-LL +                 e.insert(v2);
-LL +             },
-
-
-error: usage of `contains_key` followed by `insert` on a `HashMap`
-   |
-   |
-LL | /     if !m.contains_key(&k) {
-LL | |         foo();
-LL | |         match 0 {
-LL | |             0 if false => {
-LL | |         }
-LL | |     }
-   | |_____^
-   |
-   |
-help: try this
-   |
-LL ~     m.entry(k).or_insert_with(|| {
-LL +         foo();
-LL +         match 0 {
-LL +             0 if false => {
-LL +                 v
-LL +             },
-
-
-error: usage of `contains_key` followed by `insert` on a `HashMap`
-   |
-   |
-LL | /     if !m.contains_key(&m!(k)) {
-LL | |         m.insert(m!(k), m!(v));
-LL | |     }
-   | |_____^ help: try this: `m.entry(m!(k)).or_insert_with(|| m!(v));`
-
-error: usage of `contains_key` followed by `insert` on a `HashMap`
-   |
-   |
-LL | /     if !m.contains_key(&k) {
-LL | |         let x = (String::new(), String::new());
-LL | |         let _ = x.0;
-LL | |         m.insert(k, v);
-LL | |     }
-   | |_____^
-   |
-   |
-   |
-LL ~     m.entry(k).or_insert_with(|| {
-LL +         let x = (String::new(), String::new());
-LL +         let _ = x.0;
-LL +         v
-LL +     });
-   |
-error: aborting due to 10 previous errors
+error: aborting due to previous error
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/entry.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 #![allow(unused, clippy::needless_pass_by_value, clippy::collapsible_if)]
 #![warn(clippy::map_entry)]
 #![feature(asm)]
 use std::collections::HashMap;
 use std::hash::Hash;
 
 macro_rules! m {
 macro_rules! m {
     ($e:expr) => {{ $e }};
 
 macro_rules! insert {
 macro_rules! insert {
     ($map:expr, $key:expr, $val:expr) => {
         $map.insert($key, $val)
 }
 
 fn foo() {}
 
 
 fn hash_map<K: Eq + Hash + Copy, V: Copy>(m: &mut HashMap<K, V>, m2: &mut HashMap<K, V>, k: K, k2: K, v: V, v2: V) {
     // or_insert(v)
-    m.entry(k).or_insert(v);
+    if !m.contains_key(&k) {
+        m.insert(k, v);
 
 
     // semicolon on insert, use or_insert_with(..)
-    m.entry(k).or_insert_with(|| {
+    if !m.contains_key(&k) {
-            v
-            v
+            m.insert(k, v);
-            v2
-            v2
+            m.insert(k, v2);
-    });
+    }
 
 
     // semicolon on if, use or_insert_with(..)
-    m.entry(k).or_insert_with(|| {
+    if !m.contains_key(&k) {
-            v
-            v
+            m.insert(k, v)
-            v2
-        }
-    });
-    });
+            m.insert(k, v2)
+    }
 
 
     // early return, use if let
-    if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {
+    if !m.contains_key(&k) {
         if true {
-            e.insert(v);
+            m.insert(k, v);
         } else {
-            e.insert(v2);
+            m.insert(k, v2);
         }
     }
 
 
     // use or_insert_with(..)
-    m.entry(k).or_insert_with(|| {
+    if !m.contains_key(&k) {
         foo();
-    });
-    });
+        m.insert(k, v);
 
 
     // semicolon on insert and match, use or_insert_with(..)
-    m.entry(k).or_insert_with(|| {
+    if !m.contains_key(&k) {
         match 0 {
             1 if true => {
-                v
+                m.insert(k, v);
             _ => {
-                v2
-                v2
+                m.insert(k, v2);
-        }
-    });
+        };
+    }
+    }
 
     // one branch doesn't insert, use if let
-    if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {
+    if !m.contains_key(&k) {
         match 0 {
             0 => foo(),
             _ => {
-                e.insert(v2);
+                m.insert(k, v2);
         };
     }
 
     // use or_insert_with
     // use or_insert_with
-    m.entry(k).or_insert_with(|| {
+    if !m.contains_key(&k) {
         foo();
         match 0 {
             0 if false => {
-                v
+                m.insert(k, v);
             1 => {
                 foo();
-                v
-                v
+                m.insert(k, v);
             2 | 3 => {
                 for _ in 0..2 {
                     foo();
                 }
                 }
                 if true {
-                    v
+                    m.insert(k, v);
-                    v2
-                }
-                }
+                    m.insert(k, v2);
             },
             _ => {
-                v2
-                v2
+                m.insert(k, v2);
         }
-    });
+    }
 
 
     // ok, insert in loop
     if !m.contains_key(&k) {
         for _ in 0..2 {
             m.insert(k, v);
     }
 
 
     // macro_expansion test, use or_insert(..)
-    m.entry(m!(k)).or_insert_with(|| m!(v));
+    if !m.contains_key(&m!(k)) {
+        m.insert(m!(k), m!(v));
 
 
     // ok, map used before insertion
     if !m.contains_key(&k) {
         let _ = m.len();
         m.insert(k, v);
 
     // ok, inline asm
     // ok, inline asm
     if !m.contains_key(&k) {
         unsafe { asm!("nop") }
         m.insert(k, v);
 
 
     // ok, different keys.
     if !m.contains_key(&k) {
         m.insert(k2, v);
 
 
     // ok, different maps
     if !m.contains_key(&k) {
         m2.insert(k, v);
 
 
     // ok, insert in macro
     if !m.contains_key(&k) {
         insert!(m, k, v);
 
 
     // or_insert_with. Partial move of a local declared in the closure is ok.
-    m.entry(k).or_insert_with(|| {
+    if !m.contains_key(&k) {
         let x = (String::new(), String::new());
         let _ = x.0;
-    });
-    });
+        m.insert(k, v);
 }
 
 fn main() {}
 
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/entry.stage-id.fixed
To only update this specific test, also pass `--test-args entry.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/entry.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/entry.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/entry.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"cannot find macro `asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":2716,"byte_end":2719,"line_start":131,"line_end":131,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"        unsafe { asm!(\"nop\") }","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::asm\ncore::arch::asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `asm` in this scope\n  --> tests/ui/entry.rs:131:18\n   |\nLL |         unsafe { asm!(\"nop\") }\n   |                  ^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::asm\n           core::arch::asm\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: missing documentation for a type alias
-  --> $DIR/missing-doc.rs:9:1
+error: cannot find macro `global_asm` in this scope
    |
-LL | type Typedef = String;
-   | ^^^^^^^^^^^^^^^^^^^^^^
-   | ^^^^^^^^^^^^^^^^^^^^^^
+LL | global_asm! { "" }
    |
    |
-   = note: `-D clippy::missing-docs-in-private-items` implied by `-D warnings`
+           std::arch::global_asm
+           core::arch::global_asm
 
-error: missing documentation for a type alias
-error: missing documentation for a type alias
-  --> $DIR/missing-doc.rs:10:1
-   |
-LL | pub type PubTypedef = String;
-   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-error: missing documentation for a module
-  --> $DIR/missing-doc.rs:12:1
-   |
-   |
-LL | mod module_no_dox {}
-   | ^^^^^^^^^^^^^^^^^^^^
-error: missing documentation for a module
-  --> $DIR/missing-doc.rs:13:1
-   |
-   |
-LL | pub mod pub_module_no_dox {}
-   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-error: missing documentation for a function
-  --> $DIR/missing-doc.rs:17:1
-   |
-LL | pub fn foo2() {}
---
-   | ^^^^^^^^^^^^
-
-error: missing documentation for an enum
-  --> $DIR/missing-doc.rs:32:1
-   |
-LL | / enum Baz {
-LL | |     BazA { a: isize, b: isize },
-LL | |     BarB,
-LL | | }
-   | |_^
-error: missing documentation for a variant
-  --> $DIR/missing-doc.rs:33:5
-   |
-   |
-LL |     BazA { a: isize, b: isize },
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
-error: missing documentation for a struct field
-  --> $DIR/missing-doc.rs:33:12
-   |
-   |
-LL |     BazA { a: isize, b: isize },
-   |            ^^^^^^^^
-error: missing documentation for a struct field
-  --> $DIR/missing-doc.rs:33:22
-   |
-   |
-LL |     BazA { a: isize, b: isize },
-   |                      ^^^^^^^^
-error: missing documentation for a variant
-  --> $DIR/missing-doc.rs:34:5
-   |
-LL |     BarB,
-LL |     BarB,
-   |     ^^^^
-
-error: missing documentation for an enum
-  --> $DIR/missing-doc.rs:37:1
-   |
-LL | / pub enum PubBaz {
-LL | |     PubBazA { a: isize },
-LL | | }
-   | |_^
-error: missing documentation for a variant
-  --> $DIR/missing-doc.rs:38:5
-   |
-   |
-LL |     PubBazA { a: isize },
-   |     ^^^^^^^^^^^^^^^^^^^^
-error: missing documentation for a struct field
-  --> $DIR/missing-doc.rs:38:15
-   |
-   |
-LL |     PubBazA { a: isize },
-   |               ^^^^^^^^
-error: missing documentation for a constant
-  --> $DIR/missing-doc.rs:58:1
-   |
-LL | const FOO: u32 = 0;
---
-   | ^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: missing documentation for a static
-  --> $DIR/missing-doc.rs:67:1
-   |
-LL | static BAR: u32 = 0;
-   | ^^^^^^^^^^^^^^^^^^^^
-error: missing documentation for a static
-  --> $DIR/missing-doc.rs:74:1
-   |
-   |
-LL | pub static BAR4: u32 = 0;
-   | ^^^^^^^^^^^^^^^^^^^^^^^^^
-error: missing documentation for a module
-  --> $DIR/missing-doc.rs:76:1
-   |
-   |
-LL | / mod internal_impl {
-LL | |     /// dox
-LL | |     pub fn documented() {}
-LL | |     pub fn undocumented1() {}
-LL | |     }
-LL | | }
-   | |_^
-
---
-   |     ^^^^^^^^^^^^^^^^^^^^^
-
-error: missing documentation for a function
-  --> $DIR/missing-doc.rs:86:9
-   |
-LL |         pub fn also_undocumented1() {}
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-error: missing documentation for a function
-  --> $DIR/missing-doc.rs:87:9
-   |
-   |
-LL |         fn also_undocumented2() {}
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
-error: aborting due to 24 previous errors
+error: aborting due to previous error
 
 
---
To only update this specific test, also pass `--test-args missing-doc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/missing-doc.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/missing-doc.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/missing-doc.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"cannot find macro `global_asm` in this scope","code":null,"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2050,"byte_end":2060,"line_start":101,"line_end":101,"column_start":1,"column_end":11,"is_primary":true,"text":[{"text":"global_asm! { \"\" }","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider importing one of these items:\nstd::arch::global_asm\ncore::arch::global_asm","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `global_asm` in this scope\n  --> tests/ui/missing-doc.rs:101:1\n   |\nLL | global_asm! { \"\" }\n   | ^^^^^^^^^^\n   |\n   = note: consider importing one of these items:\n           std::arch::global_asm\n           core::arch::global_asm\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
