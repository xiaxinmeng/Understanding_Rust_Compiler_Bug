plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0fa3190394475a84360b34e074e719d519bc40f1 and b900d1485b017a027858063362b7bfec643eba02
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         m.insert(k, v);
 LL | |     } else {
 LL | |         m.insert(k, v2);
 LL | |     }
    |
    |
    = note: `-D clippy::map-entry` implied by `-D warnings`
    |
    |
 LL ~     match m.entry(k) {
 LL +         std::collections::hash_map::Entry::Vacant(e) => {
 LL +             e.insert(v);
+LL +             None;
 LL +         }
 LL +         std::collections::hash_map::Entry::Occupied(mut e) => {
-LL +             e.insert(v2);
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if m.contains_key(&k) {
 LL | |         m.insert(k, v);
 LL | |     } else {
 LL | |         m.insert(k, v2);
 LL | |     }
    |
 help: try this
    |
    |
 LL ~     match m.entry(k) {
 LL +         std::collections::hash_map::Entry::Occupied(mut e) => {
-LL +             e.insert(v);
+LL +             Some(e.insert(v));
 LL +         }
 LL +         std::collections::hash_map::Entry::Vacant(e) => {
 LL +             e.insert(v2);
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         m.insert(k, v);
 LL | |     } else {
 LL | |         foo();
 LL | |     }
    |
 help: try this
    |
    |
 LL ~     if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {
 LL +         e.insert(v);
+LL +         None;
 LL +     } else {
 LL +         foo();
 LL +     }
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         foo();
 LL | |     } else {
 LL | |         m.insert(k, v);
 LL | |     }
    |
 help: try this
    |
    |
 LL ~     if let std::collections::hash_map::Entry::Occupied(mut e) = m.entry(k) {
-LL +         e.insert(v);
+LL +         Some(e.insert(v));
 LL +     } else {
 LL +         foo();
 LL +     }
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         m.insert(k, v);
 LL | |     } else {
 LL | |         m.insert(k, v2);
 LL | |     }
    |
 help: try this
    |
    |
 LL ~     match m.entry(k) {
 LL +         std::collections::hash_map::Entry::Vacant(e) => {
 LL +             e.insert(v);
+LL +             None;
 LL +         }
 LL +         std::collections::hash_map::Entry::Occupied(mut e) => {
-LL +             e.insert(v2);
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if m.contains_key(&k) {
 LL | |         if true { m.insert(k, v) } else { m.insert(k, v2) }
 LL | |     } else {
 LL | |         m.insert(k, v)
 LL | |     };
    |
 help: try this
    |
    |
 LL ~     match m.entry(k) {
 LL +         std::collections::hash_map::Entry::Occupied(mut e) => {
 LL +             if true { Some(e.insert(v)) } else { Some(e.insert(v2)) }
 LL +         }
 LL +         std::collections::hash_map::Entry::Vacant(e) => {
 LL +             e.insert(v);
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if m.contains_key(&k) {
 LL | |         foo();
 LL | |         m.insert(k, v)
 LL | |     } else {
 LL | |         None
 LL | |     };
    |
 help: try this
    |
    |
 LL ~     if let std::collections::hash_map::Entry::Occupied(mut e) = m.entry(k) {
 LL +         foo();
 LL +         Some(e.insert(v))
 LL +     } else {
 LL +         None
 LL ~     };
 
 error: aborting due to 7 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/entry_with_else.stage-id.stderr

 // run-rustfix
 
 
 #![allow(unused, clippy::needless_pass_by_value, clippy::collapsible_if)]
 #![warn(clippy::map_entry)]
 
 use std::collections::{BTreeMap, HashMap};
 use std::hash::Hash;
 macro_rules! m {
 macro_rules! m {
     ($e:expr) => {{ $e }};
 
 fn foo() {}
 
 
 fn insert_if_absent0<K: Eq + Hash + Copy, V: Copy>(m: &mut HashMap<K, V>, k: K, v: V, v2: V) {
     match m.entry(k) {
         std::collections::hash_map::Entry::Vacant(e) => {
             e.insert(v);
+            None;
         }
         std::collections::hash_map::Entry::Occupied(mut e) => {
-            e.insert(v2);
+            Some(e.insert(v2));
     }
 
 
     match m.entry(k) {
         std::collections::hash_map::Entry::Occupied(mut e) => {
-            e.insert(v);
+            Some(e.insert(v));
         }
         std::collections::hash_map::Entry::Vacant(e) => {
             e.insert(v2);
+            None;
     }
 
 
     if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {
         e.insert(v);
+        None;
         foo();
     }
 
 
     if let std::collections::hash_map::Entry::Occupied(mut e) = m.entry(k) {
-        e.insert(v);
+        Some(e.insert(v));
         foo();
     }
 
 
     match m.entry(k) {
         std::collections::hash_map::Entry::Vacant(e) => {
             e.insert(v);
+            None;
         }
         std::collections::hash_map::Entry::Occupied(mut e) => {
-            e.insert(v2);
+            Some(e.insert(v2));
     }
 
 
     match m.entry(k) {
         std::collections::hash_map::Entry::Occupied(mut e) => {
             if true { Some(e.insert(v)) } else { Some(e.insert(v2)) }
         }
         std::collections::hash_map::Entry::Vacant(e) => {
             e.insert(v);
         }
     };
 
 
     if let std::collections::hash_map::Entry::Occupied(mut e) = m.entry(k) {
         foo();
         Some(e.insert(v))
         None
     };
 }
 
 
 fn main() {}
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/entry_with_else.stage-id.fixed
To only update this specific test, also pass `--test-args entry_with_else.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/entry_with_else.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/entry_with_else.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-a4851abe566518b2.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-a05ee8fc41b4648a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/entry_with_else.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":343,"byte_end":435,"line_start":16,"line_end":20,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        m.insert(k, v2);","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::map-entry` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":343,"byte_end":435,"line_start":16,"line_end":20,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        m.insert(k, v2);","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"match m.entry(k) {\n        std::collections::hash_map::Entry::Vacant(e) => {\n            e.insert(v);\n            None;\n        }\n        std::collections::hash_map::Entry::Occupied(mut e) => {\n            Some(e.insert(v2));\n        }\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry_with_else.rs:16:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         m.insert(k, v);\nLL | |     } else {\nLL | |         m.insert(k, v2);\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::map-entry` implied by `-D warnings`\nhelp: try this\n   |\nLL ~     match m.entry(k) {\nLL +         std::collections::hash_map::Entry::Vacant(e) => {\nLL +             e.insert(v);\nLL +             None;\nLL +         }\nLL +         std::collections::hash_map::Entry::Occupied(mut e) => {\n ...\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":441,"byte_end":532,"line_start":22,"line_end":26,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if m.contains_key(&k) {","highlight_start":5,"highlight_end":28},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        m.insert(k, v2);","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":441,"byte_end":532,"line_start":22,"line_end":26,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if m.contains_key(&k) {","highlight_start":5,"highlight_end":28},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        m.insert(k, v2);","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"match m.entry(k) {\n        std::collections::hash_map::Entry::Occupied(mut e) => {\n            Some(e.insert(v));\n        }\n        std::collections::hash_map::Entry::Vacant(e) => {\n            e.insert(v2);\n            None;\n        }\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry_with_else.rs:22:5\n   |\nLL | /     if m.contains_key(&k) {\nLL | |         m.insert(k, v);\nLL | |     } else {\nLL | |         m.insert(k, v2);\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     match m.entry(k) {\nLL +         std::collections::hash_map::Entry::Occupied(mut e) => {\nLL +             Some(e.insert(v));\nLL +         }\nLL +         std::collections::hash_map::Entry::Vacant(e) => {\nLL +             e.insert(v2);\n ...\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":538,"byte_end":620,"line_start":28,"line_end":32,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":538,"byte_end":620,"line_start":28,"line_end":32,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {\n        e.insert(v);\n        None;\n    } else {\n        foo();\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry_with_else.rs:28:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         m.insert(k, v);\nLL | |     } else {\nLL | |         foo();\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {\nLL +         e.insert(v);\nLL +         None;\nLL +     } else {\nLL +         foo();\nLL +     }\n   |\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":626,"byte_end":708,"line_start":34,"line_end":38,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":626,"byte_end":708,"line_start":34,"line_end":38,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let std::collections::hash_map::Entry::Occupied(mut e) = m.entry(k) {\n        Some(e.insert(v));\n    } else {\n        foo();\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry_with_else.rs:34:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         foo();\nLL | |     } else {\nLL | |         m.insert(k, v);\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     if let std::collections::hash_map::Entry::Occupied(mut e) = m.entry(k) {\nLL +         Some(e.insert(v));\nLL +     } else {\nLL +         foo();\nLL +     }\n   |\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":714,"byte_end":806,"line_start":40,"line_end":44,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        m.insert(k, v2);","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":714,"byte_end":806,"line_start":40,"line_end":44,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        m.insert(k, v2);","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"match m.entry(k) {\n        std::collections::hash_map::Entry::Vacant(e) => {\n            e.insert(v);\n            None;\n        }\n        std::collections::hash_map::Entry::Occupied(mut e) => {\n            Some(e.insert(v2));\n        }\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry_with_else.rs:40:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         m.insert(k, v);\nLL | |     } else {\nLL | |         m.insert(k, v2);\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     match m.entry(k) {\nLL +         std::collections::hash_map::Entry::Vacant(e) => {\nLL +             e.insert(v);\nLL +             None;\nLL +         }\nLL +         std::collections::hash_map::Entry::Occupied(mut e) => {\n ...\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":812,"byte_end":937,"line_start":46,"line_end":50,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if m.contains_key(&k) {","highlight_start":5,"highlight_end":28},{"text":"        if true { m.insert(k, v) } else { m.insert(k, v2) }","highlight_start":1,"highlight_end":60},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        m.insert(k, v)","highlight_start":1,"highlight_end":23},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":812,"byte_end":937,"line_start":46,"line_end":50,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if m.contains_key(&k) {","highlight_start":5,"highlight_end":28},{"text":"        if true { m.insert(k, v) } else { m.insert(k, v2) }","highlight_start":1,"highlight_end":60},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        m.insert(k, v)","highlight_start":1,"highlight_end":23},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"match m.entry(k) {\n        std::collections::hash_map::Entry::Occupied(mut e) => {\n            if true { Some(e.insert(v)) } else { Some(e.insert(v2)) }\n        }\n        std::collections::hash_map::Entry::Vacant(e) => {\n            e.insert(v);\n            None\n        }\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry_with_else.rs:46:5\n   |\nLL | /     if m.contains_key(&k) {\nLL | |         if true { m.insert(k, v) } else { m.insert(k, v2) }\nLL | |     } else {\nLL | |         m.insert(k, v)\nLL | |     };\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     match m.entry(k) {\nLL +         std::collections::hash_map::Entry::Occupied(mut e) => {\nLL +             if true { Some(e.insert(v)) } else { Some(e.insert(v2)) }\nLL +         }\nLL +         std::collections::hash_map::Entry::Vacant(e) => {\nLL +             e.insert(v);\n ...\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":944,"byte_end":1037,"line_start":52,"line_end":57,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if m.contains_key(&k) {","highlight_start":5,"highlight_end":28},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"        m.insert(k, v)","highlight_start":1,"highlight_end":23},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        None","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry_with_else.rs","byte_start":944,"byte_end":1037,"line_start":52,"line_end":57,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if m.contains_key(&k) {","highlight_start":5,"highlight_end":28},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"        m.insert(k, v)","highlight_start":1,"highlight_end":23},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        None","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let std::collections::hash_map::Entry::Occupied(mut e) = m.entry(k) {\n        foo();\n        Some(e.insert(v))\n    } else {\n        None\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry_with_else.rs:52:5\n   |\nLL | /     if m.contains_key(&k) {\nLL | |         foo();\nLL | |         m.insert(k, v)\nLL | |     } else {\nLL | |         None\nLL | |     };\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     if let std::collections::hash_map::Entry::Occupied(mut e) = m.entry(k) {\nLL +         foo();\nLL +         Some(e.insert(v))\nLL +     } else {\nLL +         None\nLL ~     };\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         m.insert(k, v);
 LL | |     }
    | |_____^ help: try this: `m.entry(k).or_insert(v);`
    |
    = note: `-D clippy::map-entry` implied by `-D warnings`
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         if true {
 LL | |             m.insert(k, v);
 LL | |         } else {
 LL | |             m.insert(k, v2);
 LL | |         }
 LL | |     }
    |
 help: try this
    |
    |
 LL ~     m.entry(k).or_insert_with(|| {
 LL +         if true {
 LL +             v
 LL +         } else {
 LL +             v2
 LL +         }
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         if true {
 LL | |             m.insert(k, v)
 LL | |         } else {
 LL | |             m.insert(k, v2)
 LL | |         };
 LL | |     }
    |
 help: try this
    |
    |
 LL ~     m.entry(k).or_insert_with(|| {
 LL +         if true {
 LL +             v
 LL +         } else {
 LL +             v2
 LL +         }
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         if true {
 LL | |             m.insert(k, v);
 LL | |         } else {
 LL | |         }
 LL | |     }
    | |_____^
    |
    |
 help: try this
    |
 LL ~     if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {
 LL +         if true {
 LL +             e.insert(v);
+LL +             None;
 LL +         } else {
 LL +             e.insert(v2);
-LL +             return;
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         foo();
 LL | |         m.insert(k, v);
 LL | |     }
    |
 help: try this
    |
    |
 LL ~     m.entry(k).or_insert_with(|| {
 LL +         foo();
 LL +         v
 LL +     });
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         match 0 {
 LL | |             1 if true => {
 LL | |                 m.insert(k, v);
 LL | |         };
 LL | |     }
    | |_____^
    |
    |
 help: try this
    |
 LL ~     m.entry(k).or_insert_with(|| {
 LL +         match 0 {
 LL +             1 if true => {
 LL +                 v
 LL +             },
 LL +             _ => {
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         match 0 {
 LL | |             0 => foo(),
 LL | |             _ => {
 LL | |         };
 LL | |     }
    | |_____^
    |
    |
 help: try this
    |
 LL ~     if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {
 LL +         match 0 {
 LL +             0 => foo(),
 LL +             _ => {
 LL +                 e.insert(v2);
-LL +             },
+LL +                 None;
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         foo();
 LL | |         match 0 {
 LL | |             0 if false => {
 LL | |         }
 LL | |     }
    | |_____^
    |
    |
 help: try this
    |
 LL ~     m.entry(k).or_insert_with(|| {
 LL +         foo();
 LL +         match 0 {
 LL +             0 if false => {
 LL +                 v
 LL +             },
 
 
 error: usage of `contains_key` followed by `insert` on a `HashMap`
    |
    |
 LL | /     if !m.contains_key(&m!(k)) {
 LL | |         m.insert(m!(k), m!(v));
 LL | |     }
    | |_____^ help: try this: `m.entry(m!(k)).or_insert_with(|| m!(v));`
 
 error: usage of `contains_key` followed by `insert` on a `BTreeMap`
    |
    |
 LL | /     if !m.contains_key(&k) {
 LL | |         m.insert(k, v);
 LL | |         foo();
 LL | |     }
    |
 help: try this
    |
    |
 LL ~     if let std::collections::btree_map::Entry::Vacant(e) = m.entry(k) {
 LL +         e.insert(v);
 LL +         foo();
 LL +     }
 
 error: aborting due to 10 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/entry.stage-id.stderr

 // run-rustfix
 
 
 #![allow(unused, clippy::needless_pass_by_value, clippy::collapsible_if)]
 #![warn(clippy::map_entry)]
 #![feature(asm)]
 
 use std::collections::{BTreeMap, HashMap};
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
error: test failed, to rerun pass '--test compile-test'
     // or_insert(v)
     m.entry(k).or_insert(v);
 
     // semicolon on insert, use or_insert_with(..)
     m.entry(k).or_insert_with(|| {
             v
         } else {
             v2
         }
         }
     });
 
     // semicolon on if, use or_insert_with(..)
     m.entry(k).or_insert_with(|| {
             v
         } else {
---
             },
         }
     });
 
     // ok, insert in loop
     if !m.contains_key(&k) {
         for _ in 0..2 {
             m.insert(k, v);
     }
 
 
     // macro_expansion test, use or_insert(..)
     m.entry(m!(k)).or_insert_with(|| m!(v));
 
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
 }
 
 
 fn btree_map<K: Eq + Ord + Copy, V: Copy>(m: &mut BTreeMap<K, V>, k: K, v: V, v2: V) {
     // insert then do something, use if let
     if let std::collections::btree_map::Entry::Vacant(e) = m.entry(k) {
         e.insert(v);
         foo();
 }
 
 fn main() {}
 
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/entry.stage-id.fixed
To only update this specific test, also pass `--test-args entry.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/entry.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/entry.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-a4851abe566518b2.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-a05ee8fc41b4648a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/entry.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":509,"byte_end":563,"line_start":24,"line_end":26,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::map-entry` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry.rs","byte_start":509,"byte_end":563,"line_start":24,"line_end":26,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"m.entry(k).or_insert(v);","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry.rs:24:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         m.insert(k, v);\nLL | |     }\n   | |_____^ help: try this: `m.entry(k).or_insert(v);`\n   |\n   = note: `-D clippy::map-entry` implied by `-D warnings`\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":620,"byte_end":752,"line_start":29,"line_end":35,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        if true {","highlight_start":1,"highlight_end":18},{"text":"            m.insert(k, v);","highlight_start":1,"highlight_end":28},{"text":"        } else {","highlight_start":1,"highlight_end":17},{"text":"            m.insert(k, v2);","highlight_start":1,"highlight_end":29},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry.rs","byte_start":620,"byte_end":752,"line_start":29,"line_end":35,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        if true {","highlight_start":1,"highlight_end":18},{"text":"            m.insert(k, v);","highlight_start":1,"highlight_end":28},{"text":"        } else {","highlight_start":1,"highlight_end":17},{"text":"            m.insert(k, v2);","highlight_start":1,"highlight_end":29},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"m.entry(k).or_insert_with(|| {\n        if true {\n            v\n        } else {\n            v2\n        }\n    });","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry.rs:29:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         if true {\nLL | |             m.insert(k, v);\nLL | |         } else {\nLL | |             m.insert(k, v2);\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     m.entry(k).or_insert_with(|| {\nLL +         if true {\nLL +             v\nLL +         } else {\nLL +             v2\nLL +         }\n ...\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":805,"byte_end":936,"line_start":38,"line_end":44,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        if true {","highlight_start":1,"highlight_end":18},{"text":"            m.insert(k, v)","highlight_start":1,"highlight_end":27},{"text":"        } else {","highlight_start":1,"highlight_end":17},{"text":"            m.insert(k, v2)","highlight_start":1,"highlight_end":28},{"text":"        };","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry.rs","byte_start":805,"byte_end":936,"line_start":38,"line_end":44,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        if true {","highlight_start":1,"highlight_end":18},{"text":"            m.insert(k, v)","highlight_start":1,"highlight_end":27},{"text":"        } else {","highlight_start":1,"highlight_end":17},{"text":"            m.insert(k, v2)","highlight_start":1,"highlight_end":28},{"text":"        };","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"m.entry(k).or_insert_with(|| {\n        if true {\n            v\n        } else {\n            v2\n        }\n    });","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry.rs:38:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         if true {\nLL | |             m.insert(k, v)\nLL | |         } else {\nLL | |             m.insert(k, v2)\nLL | |         };\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     m.entry(k).or_insert_with(|| {\nLL +         if true {\nLL +             v\nLL +         } else {\nLL +             v2\nLL +         }\n ...\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":974,"byte_end":1126,"line_start":47,"line_end":54,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        if true {","highlight_start":1,"highlight_end":18},{"text":"            m.insert(k, v);","highlight_start":1,"highlight_end":28},{"text":"        } else {","highlight_start":1,"highlight_end":17},{"text":"            m.insert(k, v2);","highlight_start":1,"highlight_end":29},{"text":"            return;","highlight_start":1,"highlight_end":20},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry.rs","byte_start":974,"byte_end":1126,"line_start":47,"line_end":54,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        if true {","highlight_start":1,"highlight_end":18},{"text":"            m.insert(k, v);","highlight_start":1,"highlight_end":28},{"text":"        } else {","highlight_start":1,"highlight_end":17},{"text":"            m.insert(k, v2);","highlight_start":1,"highlight_end":29},{"text":"            return;","highlight_start":1,"highlight_end":20},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {\n        if true {\n            e.insert(v);\n            None;\n        } else {\n            e.insert(v2);\n            None;\n            return;\n        }\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry.rs:47:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         if true {\nLL | |             m.insert(k, v);\nLL | |         } else {\n...  |\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {\nLL +         if true {\nLL +             e.insert(v);\nLL +             None;\nLL +         } else {\nLL +             e.insert(v2);\n ...\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":1162,"byte_end":1231,"line_start":57,"line_end":60,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry.rs","byte_start":1162,"byte_end":1231,"line_start":57,"line_end":60,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"m.entry(k).or_insert_with(|| {\n        foo();\n        v\n    });","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry.rs:57:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         foo();\nLL | |         m.insert(k, v);\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     m.entry(k).or_insert_with(|| {\nLL +         foo();\nLL +         v\nLL +     });\n   |\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":1298,"byte_end":1498,"line_start":63,"line_end":72,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        match 0 {","highlight_start":1,"highlight_end":18},{"text":"            1 if true => {","highlight_start":1,"highlight_end":27},{"text":"                m.insert(k, v);","highlight_start":1,"highlight_end":32},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"            _ => {","highlight_start":1,"highlight_end":19},{"text":"                m.insert(k, v2);","highlight_start":1,"highlight_end":33},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"        };","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry.rs","byte_start":1298,"byte_end":1498,"line_start":63,"line_end":72,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        match 0 {","highlight_start":1,"highlight_end":18},{"text":"            1 if true => {","highlight_start":1,"highlight_end":27},{"text":"                m.insert(k, v);","highlight_start":1,"highlight_end":32},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"            _ => {","highlight_start":1,"highlight_end":19},{"text":"                m.insert(k, v2);","highlight_start":1,"highlight_end":33},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"        };","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"m.entry(k).or_insert_with(|| {\n        match 0 {\n            1 if true => {\n                v\n            },\n            _ => {\n                v2\n            },\n        }\n    });","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry.rs:63:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         match 0 {\nLL | |             1 if true => {\nLL | |                 m.insert(k, v);\n...  |\nLL | |         };\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     m.entry(k).or_insert_with(|| {\nLL +         match 0 {\nLL +             1 if true => {\nLL +                 v\nLL +             },\nLL +             _ => {\n ...\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":1549,"byte_end":1699,"line_start":75,"line_end":82,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        match 0 {","highlight_start":1,"highlight_end":18},{"text":"            0 => foo(),","highlight_start":1,"highlight_end":24},{"text":"            _ => {","highlight_start":1,"highlight_end":19},{"text":"                m.insert(k, v2);","highlight_start":1,"highlight_end":33},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"        };","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry.rs","byte_start":1549,"byte_end":1699,"line_start":75,"line_end":82,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        match 0 {","highlight_start":1,"highlight_end":18},{"text":"            0 => foo(),","highlight_start":1,"highlight_end":24},{"text":"            _ => {","highlight_start":1,"highlight_end":19},{"text":"                m.insert(k, v2);","highlight_start":1,"highlight_end":33},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"        };","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {\n        match 0 {\n            0 => foo(),\n            _ => {\n                e.insert(v2);\n                None;\n            },\n        };\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry.rs:75:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         match 0 {\nLL | |             0 => foo(),\nLL | |             _ => {\n...  |\nLL | |         };\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     if let std::collections::hash_map::Entry::Vacant(e) = m.entry(k) {\nLL +         match 0 {\nLL +             0 => foo(),\nLL +             _ => {\nLL +                 e.insert(v2);\nLL +                 None;\n ...\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":1731,"byte_end":2293,"line_start":85,"line_end":109,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"        match 0 {","highlight_start":1,"highlight_end":18},{"text":"            0 if false => {","highlight_start":1,"highlight_end":28},{"text":"                m.insert(k, v);","highlight_start":1,"highlight_end":32},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"            1 => {","highlight_start":1,"highlight_end":19},{"text":"                foo();","highlight_start":1,"highlight_end":23},{"text":"                m.insert(k, v);","highlight_start":1,"highlight_end":32},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"            2 | 3 => {","highlight_start":1,"highlight_end":23},{"text":"                for _ in 0..2 {","highlight_start":1,"highlight_end":32},{"text":"                    foo();","highlight_start":1,"highlight_end":27},{"text":"                }","highlight_start":1,"highlight_end":18},{"text":"                if true {","highlight_start":1,"highlight_end":26},{"text":"                    m.insert(k, v);","highlight_start":1,"highlight_end":36},{"text":"                } else {","highlight_start":1,"highlight_end":25},{"text":"                    m.insert(k, v2);","highlight_start":1,"highlight_end":37},{"text":"                };","highlight_start":1,"highlight_end":19},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"            _ => {","highlight_start":1,"highlight_end":19},{"text":"                m.insert(k, v2);","highlight_start":1,"highlight_end":33},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry.rs","byte_start":1731,"byte_end":2293,"line_start":85,"line_end":109,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"        match 0 {","highlight_start":1,"highlight_end":18},{"text":"            0 if false => {","highlight_start":1,"highlight_end":28},{"text":"                m.insert(k, v);","highlight_start":1,"highlight_end":32},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"            1 => {","highlight_start":1,"highlight_end":19},{"text":"                foo();","highlight_start":1,"highlight_end":23},{"text":"                m.insert(k, v);","highlight_start":1,"highlight_end":32},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"            2 | 3 => {","highlight_start":1,"highlight_end":23},{"text":"                for _ in 0..2 {","highlight_start":1,"highlight_end":32},{"text":"                    foo();","highlight_start":1,"highlight_end":27},{"text":"                }","highlight_start":1,"highlight_end":18},{"text":"                if true {","highlight_start":1,"highlight_end":26},{"text":"                    m.insert(k, v);","highlight_start":1,"highlight_end":36},{"text":"                } else {","highlight_start":1,"highlight_end":25},{"text":"                    m.insert(k, v2);","highlight_start":1,"highlight_end":37},{"text":"                };","highlight_start":1,"highlight_end":19},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"            _ => {","highlight_start":1,"highlight_end":19},{"text":"                m.insert(k, v2);","highlight_start":1,"highlight_end":33},{"text":"            },","highlight_start":1,"highlight_end":15},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"m.entry(k).or_insert_with(|| {\n        foo();\n        match 0 {\n            0 if false => {\n                v\n            },\n            1 => {\n                foo();\n                v\n            },\n            2 | 3 => {\n                for _ in 0..2 {\n                    foo();\n                }\n                if true {\n                    v\n                } else {\n                    v2\n                }\n            },\n            _ => {\n                v2\n            },\n        }\n    });","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry.rs:85:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         foo();\nLL | |         match 0 {\nLL | |             0 if false => {\n...  |\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     m.entry(k).or_insert_with(|| {\nLL +         foo();\nLL +         match 0 {\nLL +             0 if false => {\nLL +                 v\nLL +             },\n ...\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `HashMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":2470,"byte_end":2536,"line_start":119,"line_end":121,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&m!(k)) {","highlight_start":5,"highlight_end":33},{"text":"        m.insert(m!(k), m!(v));","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry.rs","byte_start":2470,"byte_end":2536,"line_start":119,"line_end":121,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&m!(k)) {","highlight_start":5,"highlight_end":33},{"text":"        m.insert(m!(k), m!(v));","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"m.entry(m!(k)).or_insert_with(|| m!(v));","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `HashMap`\n  --> tests/ui/entry.rs:119:5\n   |\nLL | /     if !m.contains_key(&m!(k)) {\nLL | |         m.insert(m!(k), m!(v));\nLL | |     }\n   | |_____^ help: try this: `m.entry(m!(k)).or_insert_with(|| m!(v));`\n\n"}
{"message":"usage of `contains_key` followed by `insert` on a `BTreeMap`","code":{"code":"clippy::map_entry","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/entry.rs","byte_start":3174,"byte_end":3243,"line_start":153,"line_end":156,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/entry.rs","byte_start":3174,"byte_end":3243,"line_start":153,"line_end":156,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !m.contains_key(&k) {","highlight_start":5,"highlight_end":29},{"text":"        m.insert(k, v);","highlight_start":1,"highlight_end":24},{"text":"        foo();","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let std::collections::btree_map::Entry::Vacant(e) = m.entry(k) {\n        e.insert(v);\n        foo();\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: usage of `contains_key` followed by `insert` on a `BTreeMap`\n  --> tests/ui/entry.rs:153:5\n   |\nLL | /     if !m.contains_key(&k) {\nLL | |         m.insert(k, v);\nLL | |         foo();\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL ~     if let std::collections::btree_map::Entry::Vacant(e) = m.entry(k) {\nLL +         e.insert(v);\nLL +         foo();\nLL +     }\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: redundant pattern matching, consider using `is_ok()`
    |
    |
 LL |     if let Ok(_) = m.lock() {}
    |     -------^^^^^----------- help: try this: `if m.lock().is_ok()`
    |
    = note: `-D clippy::redundant-pattern-matching` implied by `-D warnings`
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_err()`
    |
    |
 LL |     if let Err(_) = Err::<(), _>(m.lock().unwrap().0) {}
    |     -------^^^^^^------------------------------------ help: try this: `if Err::<(), _>(m.lock().unwrap().0).is_err()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_ok()`
    |
    |
 LL |         if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {}
    |         -------^^^^^----------------------------------------- help: try this: `if Ok::<_, std::sync::MutexGuard<()>>(()).is_ok()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_ok()`
    |
    |
 LL |     if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {
    |     -------^^^^^----------------------------------------- help: try this: `if Ok::<_, std::sync::MutexGuard<()>>(()).is_ok()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_ok()`
    |
    |
 LL |     if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {}
    |     -------^^^^^----------------------------------------- help: try this: `if Ok::<_, std::sync::MutexGuard<()>>(()).is_ok()`
+   |
+   = note: this will change drop order of the result, as well as all temporaries
+   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_err()`
    |
    |
 LL |     if let Err(_) = Err::<std::sync::MutexGuard<()>, _>(()) {}
    |     -------^^^^^^------------------------------------------ help: try this: `if Err::<std::sync::MutexGuard<()>, _>(()).is_err()`
+   |
+   = note: this will change drop order of the result, as well as all temporaries
+   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_ok()`
    |
    |
 LL |     if let Ok(_) = Ok::<_, ()>(String::new()) {}
    |     -------^^^^^----------------------------- help: try this: `if Ok::<_, ()>(String::new()).is_ok()`
 
 error: redundant pattern matching, consider using `is_err()`
    |
    |
 LL |     if let Err(_) = Err::<(), _>((String::new(), ())) {}
    |     -------^^^^^^------------------------------------ help: try this: `if Err::<(), _>((String::new(), ())).is_err()`
 
 error: redundant pattern matching, consider using `is_some()`
    |
    |
 LL |     if let Some(_) = Some(m.lock()) {}
    |     -------^^^^^^^----------------- help: try this: `if Some(m.lock()).is_some()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_some()`
    |
    |
 LL |     if let Some(_) = Some(m.lock().unwrap().0) {}
    |     -------^^^^^^^---------------------------- help: try this: `if Some(m.lock().unwrap().0).is_some()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_none()`
    |
    |
 LL |         if let None = None::<std::sync::MutexGuard<()>> {}
    |         -------^^^^------------------------------------ help: try this: `if None::<std::sync::MutexGuard<()>>.is_none()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_none()`
    |
    |
 LL |     if let None = None::<std::sync::MutexGuard<()>> {
    |     -------^^^^------------------------------------ help: try this: `if None::<std::sync::MutexGuard<()>>.is_none()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_none()`
    |
    |
 LL |     if let None = None::<std::sync::MutexGuard<()>> {}
    |     -------^^^^------------------------------------ help: try this: `if None::<std::sync::MutexGuard<()>>.is_none()`
+   |
+   = note: this will change drop order of the result, as well as all temporaries
+   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_some()`
    |
    |
 LL |     if let Some(_) = Some(String::new()) {}
    |     -------^^^^^^^---------------------- help: try this: `if Some(String::new()).is_some()`
 
 error: redundant pattern matching, consider using `is_some()`
    |
    |
 LL |     if let Some(_) = Some((String::new(), ())) {}
    |     -------^^^^^^^---------------------------- help: try this: `if Some((String::new(), ())).is_some()`
 
 error: redundant pattern matching, consider using `is_ready()`
    |
    |
 LL |     if let Ready(_) = Ready(m.lock()) {}
    |     -------^^^^^^^^------------------ help: try this: `if Ready(m.lock()).is_ready()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_ready()`
    |
    |
 LL |     if let Ready(_) = Ready(m.lock().unwrap().0) {}
    |     -------^^^^^^^^----------------------------- help: try this: `if Ready(m.lock().unwrap().0).is_ready()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_pending()`
    |
    |
 LL |         if let Pending = Pending::<std::sync::MutexGuard<()>> {}
    |         -------^^^^^^^--------------------------------------- help: try this: `if Pending::<std::sync::MutexGuard<()>>.is_pending()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_pending()`
    |
    |
 LL |     if let Pending = Pending::<std::sync::MutexGuard<()>> {
    |     -------^^^^^^^--------------------------------------- help: try this: `if Pending::<std::sync::MutexGuard<()>>.is_pending()`
    |
    = note: this will change drop order of the result, as well as all temporaries
    = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_pending()`
    |
    |
 LL |     if let Pending = Pending::<std::sync::MutexGuard<()>> {}
    |     -------^^^^^^^--------------------------------------- help: try this: `if Pending::<std::sync::MutexGuard<()>>.is_pending()`
+   |
+   = note: this will change drop order of the result, as well as all temporaries
+   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important
 
 error: redundant pattern matching, consider using `is_ready()`
    |
    |
 LL |     if let Ready(_) = Ready(String::new()) {}
    |     -------^^^^^^^^----------------------- help: try this: `if Ready(String::new()).is_ready()`
 
 error: redundant pattern matching, consider using `is_ready()`
    |
    |
 LL |     if let Ready(_) = Ready((String::new(), ())) {}
    |     -------^^^^^^^^----------------------------- help: try this: `if Ready((String::new(), ())).is_ready()`
 error: aborting due to 22 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/redundant_pattern_matching_drop_order.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args redundant_pattern_matching_drop_order.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/redundant_pattern_matching_drop_order.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/redundant_pattern_matching_drop_order.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-a4851abe566518b2.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-a05ee8fc41b4648a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/redundant_pattern_matching_drop_order.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"redundant pattern matching, consider using `is_ok()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":234,"byte_end":239,"line_start":12,"line_end":12,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"    if let Ok(_) = m.lock() {}","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::redundant-pattern-matching` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":227,"byte_end":250,"line_start":12,"line_end":12,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    if let Ok(_) = m.lock() {}","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":"if m.lock().is_ok()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_ok()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:12:12\n   |\nLL |     if let Ok(_) = m.lock() {}\n   |     -------^^^^^----------- help: try this: `if m.lock().is_ok()`\n   |\n   = note: `-D clippy::redundant-pattern-matching` implied by `-D warnings`\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_err()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":265,"byte_end":271,"line_start":13,"line_end":13,"column_start":12,"column_end":18,"is_primary":true,"text":[{"text":"    if let Err(_) = Err::<(), _>(m.lock().unwrap().0) {}","highlight_start":12,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":258,"byte_end":307,"line_start":13,"line_end":13,"column_start":5,"column_end":54,"is_primary":true,"text":[{"text":"    if let Err(_) = Err::<(), _>(m.lock().unwrap().0) {}","highlight_start":5,"highlight_end":54}],"label":null,"suggested_replacement":"if Err::<(), _>(m.lock().unwrap().0).is_err()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_err()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:13:12\n   |\nLL |     if let Err(_) = Err::<(), _>(m.lock().unwrap().0) {}\n   |     -------^^^^^^------------------------------------ help: try this: `if Err::<(), _>(m.lock().unwrap().0).is_err()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_ok()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":333,"byte_end":338,"line_start":16,"line_end":16,"column_start":16,"column_end":21,"is_primary":true,"text":[{"text":"        if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {}","highlight_start":16,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":326,"byte_end":379,"line_start":16,"line_end":16,"column_start":9,"column_end":62,"is_primary":true,"text":[{"text":"        if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {}","highlight_start":9,"highlight_end":62}],"label":null,"suggested_replacement":"if Ok::<_, std::sync::MutexGuard<()>>(()).is_ok()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_ok()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:16:16\n   |\nLL |         if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {}\n   |         -------^^^^^----------------------------------------- help: try this: `if Ok::<_, std::sync::MutexGuard<()>>(()).is_ok()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_ok()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":400,"byte_end":405,"line_start":18,"line_end":18,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"    if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":393,"byte_end":446,"line_start":18,"line_end":18,"column_start":5,"column_end":58,"is_primary":true,"text":[{"text":"    if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {","highlight_start":5,"highlight_end":58}],"label":null,"suggested_replacement":"if Ok::<_, std::sync::MutexGuard<()>>(()).is_ok()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_ok()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:18:12\n   |\nLL |     if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {\n   |     -------^^^^^----------------------------------------- help: try this: `if Ok::<_, std::sync::MutexGuard<()>>(()).is_ok()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_ok()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":479,"byte_end":484,"line_start":21,"line_end":21,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"    if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {}","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":472,"byte_end":525,"line_start":21,"line_end":21,"column_start":5,"column_end":58,"is_primary":true,"text":[{"text":"    if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {}","highlight_start":5,"highlight_end":58}],"label":null,"suggested_replacement":"if Ok::<_, std::sync::MutexGuard<()>>(()).is_ok()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_ok()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:21:12\n   |\nLL |     if let Ok(_) = Ok::<_, std::sync::MutexGuard<()>>(()) {}\n   |     -------^^^^^----------------------------------------- help: try this: `if Ok::<_, std::sync::MutexGuard<()>>(()).is_ok()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_err()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":540,"byte_end":546,"line_start":22,"line_end":22,"column_start":12,"column_end":18,"is_primary":true,"text":[{"text":"    if let Err(_) = Err::<std::sync::MutexGuard<()>, _>(()) {}","highlight_start":12,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":533,"byte_end":588,"line_start":22,"line_end":22,"column_start":5,"column_end":60,"is_primary":true,"text":[{"text":"    if let Err(_) = Err::<std::sync::MutexGuard<()>, _>(()) {}","highlight_start":5,"highlight_end":60}],"label":null,"suggested_replacement":"if Err::<std::sync::MutexGuard<()>, _>(()).is_err()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_err()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:22:12\n   |\nLL |     if let Err(_) = Err::<std::sync::MutexGuard<()>, _>(()) {}\n   |     -------^^^^^^------------------------------------------ help: try this: `if Err::<std::sync::MutexGuard<()>, _>(()).is_err()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_ok()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":604,"byte_end":609,"line_start":24,"line_end":24,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"    if let Ok(_) = Ok::<_, ()>(String::new()) {}","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":597,"byte_end":638,"line_start":24,"line_end":24,"column_start":5,"column_end":46,"is_primary":true,"text":[{"text":"    if let Ok(_) = Ok::<_, ()>(String::new()) {}","highlight_start":5,"highlight_end":46}],"label":null,"suggested_replacement":"if Ok::<_, ()>(String::new()).is_ok()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_ok()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:24:12\n   |\nLL |     if let Ok(_) = Ok::<_, ()>(String::new()) {}\n   |     -------^^^^^----------------------------- help: try this: `if Ok::<_, ()>(String::new()).is_ok()`\n\n"}
{"message":"redundant pattern matching, consider using `is_err()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":653,"byte_end":659,"line_start":25,"line_end":25,"column_start":12,"column_end":18,"is_primary":true,"text":[{"text":"    if let Err(_) = Err::<(), _>((String::new(), ())) {}","highlight_start":12,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":646,"byte_end":695,"line_start":25,"line_end":25,"column_start":5,"column_end":54,"is_primary":true,"text":[{"text":"    if let Err(_) = Err::<(), _>((String::new(), ())) {}","highlight_start":5,"highlight_end":54}],"label":null,"suggested_replacement":"if Err::<(), _>((String::new(), ())).is_err()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_err()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:25:12\n   |\nLL |     if let Err(_) = Err::<(), _>((String::new(), ())) {}\n   |     -------^^^^^^------------------------------------ help: try this: `if Err::<(), _>((String::new(), ())).is_err()`\n\n"}
{"message":"redundant pattern matching, consider using `is_some()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":725,"byte_end":732,"line_start":28,"line_end":28,"column_start":12,"column_end":19,"is_primary":true,"text":[{"text":"    if let Some(_) = Some(m.lock()) {}","highlight_start":12,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":718,"byte_end":749,"line_start":28,"line_end":28,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    if let Some(_) = Some(m.lock()) {}","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":"if Some(m.lock()).is_some()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_some()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:28:12\n   |\nLL |     if let Some(_) = Some(m.lock()) {}\n   |     -------^^^^^^^----------------- help: try this: `if Some(m.lock()).is_some()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_some()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":764,"byte_end":771,"line_start":29,"line_end":29,"column_start":12,"column_end":19,"is_primary":true,"text":[{"text":"    if let Some(_) = Some(m.lock().unwrap().0) {}","highlight_start":12,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":757,"byte_end":799,"line_start":29,"line_end":29,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    if let Some(_) = Some(m.lock().unwrap().0) {}","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":"if Some(m.lock().unwrap().0).is_some()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_some()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:29:12\n   |\nLL |     if let Some(_) = Some(m.lock().unwrap().0) {}\n   |     -------^^^^^^^---------------------------- help: try this: `if Some(m.lock().unwrap().0).is_some()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_none()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":825,"byte_end":829,"line_start":32,"line_end":32,"column_start":16,"column_end":20,"is_primary":true,"text":[{"text":"        if let None = None::<std::sync::MutexGuard<()>> {}","highlight_start":16,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":818,"byte_end":865,"line_start":32,"line_end":32,"column_start":9,"column_end":56,"is_primary":true,"text":[{"text":"        if let None = None::<std::sync::MutexGuard<()>> {}","highlight_start":9,"highlight_end":56}],"label":null,"suggested_replacement":"if None::<std::sync::MutexGuard<()>>.is_none()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_none()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:32:16\n   |\nLL |         if let None = None::<std::sync::MutexGuard<()>> {}\n   |         -------^^^^------------------------------------ help: try this: `if None::<std::sync::MutexGuard<()>>.is_none()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_none()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":886,"byte_end":890,"line_start":34,"line_end":34,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"    if let None = None::<std::sync::MutexGuard<()>> {","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":879,"byte_end":926,"line_start":34,"line_end":34,"column_start":5,"column_end":52,"is_primary":true,"text":[{"text":"    if let None = None::<std::sync::MutexGuard<()>> {","highlight_start":5,"highlight_end":52}],"label":null,"suggested_replacement":"if None::<std::sync::MutexGuard<()>>.is_none()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_none()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:34:12\n   |\nLL |     if let None = None::<std::sync::MutexGuard<()>> {\n   |     -------^^^^------------------------------------ help: try this: `if None::<std::sync::MutexGuard<()>>.is_none()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_none()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":960,"byte_end":964,"line_start":38,"line_end":38,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"    if let None = None::<std::sync::MutexGuard<()>> {}","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":953,"byte_end":1000,"line_start":38,"line_end":38,"column_start":5,"column_end":52,"is_primary":true,"text":[{"text":"    if let None = None::<std::sync::MutexGuard<()>> {}","highlight_start":5,"highlight_end":52}],"label":null,"suggested_replacement":"if None::<std::sync::MutexGuard<()>>.is_none()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_none()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:38:12\n   |\nLL |     if let None = None::<std::sync::MutexGuard<()>> {}\n   |     -------^^^^------------------------------------ help: try this: `if None::<std::sync::MutexGuard<()>>.is_none()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_some()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1016,"byte_end":1023,"line_start":40,"line_end":40,"column_start":12,"column_end":19,"is_primary":true,"text":[{"text":"    if let Some(_) = Some(String::new()) {}","highlight_start":12,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1009,"byte_end":1045,"line_start":40,"line_end":40,"column_start":5,"column_end":41,"is_primary":true,"text":[{"text":"    if let Some(_) = Some(String::new()) {}","highlight_start":5,"highlight_end":41}],"label":null,"suggested_replacement":"if Some(String::new()).is_some()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_some()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:40:12\n   |\nLL |     if let Some(_) = Some(String::new()) {}\n   |     -------^^^^^^^---------------------- help: try this: `if Some(String::new()).is_some()`\n\n"}
{"message":"redundant pattern matching, consider using `is_some()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1060,"byte_end":1067,"line_start":41,"line_end":41,"column_start":12,"column_end":19,"is_primary":true,"text":[{"text":"    if let Some(_) = Some((String::new(), ())) {}","highlight_start":12,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1053,"byte_end":1095,"line_start":41,"line_end":41,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    if let Some(_) = Some((String::new(), ())) {}","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":"if Some((String::new(), ())).is_some()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_some()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:41:12\n   |\nLL |     if let Some(_) = Some((String::new(), ())) {}\n   |     -------^^^^^^^---------------------------- help: try this: `if Some((String::new(), ())).is_some()`\n\n"}
{"message":"redundant pattern matching, consider using `is_ready()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1123,"byte_end":1131,"line_start":44,"line_end":44,"column_start":12,"column_end":20,"is_primary":true,"text":[{"text":"    if let Ready(_) = Ready(m.lock()) {}","highlight_start":12,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1116,"byte_end":1149,"line_start":44,"line_end":44,"column_start":5,"column_end":38,"is_primary":true,"text":[{"text":"    if let Ready(_) = Ready(m.lock()) {}","highlight_start":5,"highlight_end":38}],"label":null,"suggested_replacement":"if Ready(m.lock()).is_ready()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_ready()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:44:12\n   |\nLL |     if let Ready(_) = Ready(m.lock()) {}\n   |     -------^^^^^^^^------------------ help: try this: `if Ready(m.lock()).is_ready()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_ready()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1164,"byte_end":1172,"line_start":45,"line_end":45,"column_start":12,"column_end":20,"is_primary":true,"text":[{"text":"    if let Ready(_) = Ready(m.lock().unwrap().0) {}","highlight_start":12,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1157,"byte_end":1201,"line_start":45,"line_end":45,"column_start":5,"column_end":49,"is_primary":true,"text":[{"text":"    if let Ready(_) = Ready(m.lock().unwrap().0) {}","highlight_start":5,"highlight_end":49}],"label":null,"suggested_replacement":"if Ready(m.lock().unwrap().0).is_ready()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_ready()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:45:12\n   |\nLL |     if let Ready(_) = Ready(m.lock().unwrap().0) {}\n   |     -------^^^^^^^^----------------------------- help: try this: `if Ready(m.lock().unwrap().0).is_ready()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_pending()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1227,"byte_end":1234,"line_start":48,"line_end":48,"column_start":16,"column_end":23,"is_primary":true,"text":[{"text":"        if let Pending = Pending::<std::sync::MutexGuard<()>> {}","highlight_start":16,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1220,"byte_end":1273,"line_start":48,"line_end":48,"column_start":9,"column_end":62,"is_primary":true,"text":[{"text":"        if let Pending = Pending::<std::sync::MutexGuard<()>> {}","highlight_start":9,"highlight_end":62}],"label":null,"suggested_replacement":"if Pending::<std::sync::MutexGuard<()>>.is_pending()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_pending()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:48:16\n   |\nLL |         if let Pending = Pending::<std::sync::MutexGuard<()>> {}\n   |         -------^^^^^^^--------------------------------------- help: try this: `if Pending::<std::sync::MutexGuard<()>>.is_pending()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_pending()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1294,"byte_end":1301,"line_start":50,"line_end":50,"column_start":12,"column_end":19,"is_primary":true,"text":[{"text":"    if let Pending = Pending::<std::sync::MutexGuard<()>> {","highlight_start":12,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1287,"byte_end":1340,"line_start":50,"line_end":50,"column_start":5,"column_end":58,"is_primary":true,"text":[{"text":"    if let Pending = Pending::<std::sync::MutexGuard<()>> {","highlight_start":5,"highlight_end":58}],"label":null,"suggested_replacement":"if Pending::<std::sync::MutexGuard<()>>.is_pending()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_pending()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:50:12\n   |\nLL |     if let Pending = Pending::<std::sync::MutexGuard<()>> {\n   |     -------^^^^^^^--------------------------------------- help: try this: `if Pending::<std::sync::MutexGuard<()>>.is_pending()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_pending()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1374,"byte_end":1381,"line_start":54,"line_end":54,"column_start":12,"column_end":19,"is_primary":true,"text":[{"text":"    if let Pending = Pending::<std::sync::MutexGuard<()>> {}","highlight_start":12,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this will change drop order of the result, as well as all temporaries","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#[allow(clippy::redundant_pattern_matching)]` if this is important","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1367,"byte_end":1420,"line_start":54,"line_end":54,"column_start":5,"column_end":58,"is_primary":true,"text":[{"text":"    if let Pending = Pending::<std::sync::MutexGuard<()>> {}","highlight_start":5,"highlight_end":58}],"label":null,"suggested_replacement":"if Pending::<std::sync::MutexGuard<()>>.is_pending()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_pending()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:54:12\n   |\nLL |     if let Pending = Pending::<std::sync::MutexGuard<()>> {}\n   |     -------^^^^^^^--------------------------------------- help: try this: `if Pending::<std::sync::MutexGuard<()>>.is_pending()`\n   |\n   = note: this will change drop order of the result, as well as all temporaries\n   = note: add `#[allow(clippy::redundant_pattern_matching)]` if this is important\n\n"}
{"message":"redundant pattern matching, consider using `is_ready()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1436,"byte_end":1444,"line_start":56,"line_end":56,"column_start":12,"column_end":20,"is_primary":true,"text":[{"text":"    if let Ready(_) = Ready(String::new()) {}","highlight_start":12,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1429,"byte_end":1467,"line_start":56,"line_end":56,"column_start":5,"column_end":43,"is_primary":true,"text":[{"text":"    if let Ready(_) = Ready(String::new()) {}","highlight_start":5,"highlight_end":43}],"label":null,"suggested_replacement":"if Ready(String::new()).is_ready()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_ready()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:56:12\n   |\nLL |     if let Ready(_) = Ready(String::new()) {}\n   |     -------^^^^^^^^----------------------- help: try this: `if Ready(String::new()).is_ready()`\n\n"}
{"message":"redundant pattern matching, consider using `is_ready()`","code":{"code":"clippy::redundant_pattern_matching","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1482,"byte_end":1490,"line_start":57,"line_end":57,"column_start":12,"column_end":20,"is_primary":true,"text":[{"text":"    if let Ready(_) = Ready((String::new(), ())) {}","highlight_start":12,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_pattern_matching_drop_order.rs","byte_start":1475,"byte_end":1519,"line_start":57,"line_end":57,"column_start":5,"column_end":49,"is_primary":true,"text":[{"text":"    if let Ready(_) = Ready((String::new(), ())) {}","highlight_start":5,"highlight_end":49}],"label":null,"suggested_replacement":"if Ready((String::new(), ())).is_ready()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant pattern matching, consider using `is_ready()`\n  --> tests/ui/redundant_pattern_matching_drop_order.rs:57:12\n   |\nLL |     if let Ready(_) = Ready((String::new(), ())) {}\n   |     -------^^^^^^^^----------------------------- help: try this: `if Ready((String::new(), ())).is_ready()`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
