plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between d45ed7502ad225739270a368528725930f54b7b6 and bbd28f78ae6ad4a20be0586cea0e41673612081b
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if !a.is_empty() {
 LL | |         panic!("qaqaq{:?}", a);
 LL | |     }
-   | |_____^ help: try: `assert!(a.is_empty(), "qaqaq{:?}", a);`
+   | |_____^ help: try: `assert!(a.is_empty(), $crate::const_format_args!($fmt, $($arg)+));`
    |
    = note: `-D clippy::if-then-panic` implied by `-D warnings`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if !a.is_empty() {
 LL | |         panic!("qwqwq");
 LL | |     }
    | |_____^ help: try: `assert!(a.is_empty(), "qwqwq");`
 
 error: only a `panic!` in `if`-then statement
error: test failed, to rerun pass '--test compile-test'
    |
    |
 LL | /     if b.is_empty() {
 LL | |         panic!("panic1");
 LL | |     }
    | |_____^ help: try: `assert!(!b.is_empty(), "panic1");`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if b.is_empty() && a.is_empty() {
 LL | |         panic!("panic2");
 LL | |     }
    | |_____^ help: try: `assert!(!(b.is_empty() && a.is_empty()), "panic2");`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if a.is_empty() && !b.is_empty() {
 LL | |         panic!("panic3");
 LL | |     }
    | |_____^ help: try: `assert!(!(a.is_empty() && !b.is_empty()), "panic3");`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if b.is_empty() || a.is_empty() {
 LL | |         panic!("panic4");
 LL | |     }
    | |_____^ help: try: `assert!(!(b.is_empty() || a.is_empty()), "panic4");`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if a.is_empty() || !b.is_empty() {
 LL | |         panic!("panic5");
 LL | |     }
    | |_____^ help: try: `assert!(!(a.is_empty() || !b.is_empty()), "panic5");`
 error: aborting due to 7 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/if_then_panic.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![warn(clippy::if_then_panic)]
 fn main() {
 fn main() {
     let a = vec![1, 2, 3];
     let c = Some(2);
     if !a.is_empty()
         && a.len() == 3
         && c != None
         && !a.is_empty()
         && a.len() == 3
         && !a.is_empty()
         && a.len() == 3
         && !a.is_empty()
         && a.len() == 3
     {
         panic!("qaqaq{:?}", a);
     }
-    assert!(a.is_empty(), "qaqaq{:?}", a);
+    assert!(a.is_empty(), $crate::const_format_args!($fmt, $($arg)+));
     assert!(a.is_empty(), "qwqwq");
     if a.len() == 3 {
         println!("qwq");
         println!("qwq");
         println!("qwq");
     }
     if let Some(b) = c {
         panic!("orz {}", b);
     }
     if a.len() == 3 {
         panic!("qaqaq");
     } else {
         println!("qwq");
     }
     let b = vec![1, 2, 3];
     assert!(!b.is_empty(), "panic1");
     assert!(!(b.is_empty() && a.is_empty()), "panic2");
     assert!(!(a.is_empty() && !b.is_empty()), "panic3");
     assert!(!(b.is_empty() || a.is_empty()), "panic4");
     assert!(!(a.is_empty() || !b.is_empty()), "panic5");
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/if_then_panic.stage-id.fixed
To only update this specific test, also pass `--test-args if_then_panic.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/if_then_panic.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/if_then_panic.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/if_then_panic.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"only a `panic!` in `if`-then statement","code":{"code":"clippy::if_then_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":369,"byte_end":425,"line_start":19,"line_end":21,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !a.is_empty() {","highlight_start":5,"highlight_end":23},{"text":"        panic!(\"qaqaq{:?}\", a);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::if-then-panic` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":369,"byte_end":425,"line_start":19,"line_end":21,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !a.is_empty() {","highlight_start":5,"highlight_end":23},{"text":"        panic!(\"qaqaq{:?}\", a);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"assert!(a.is_empty(), $crate::const_format_args!($fmt, $($arg)+));","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: only a `panic!` in `if`-then statement\n  --> tests/ui/if_then_panic.rs:19:5\n   |\nLL | /     if !a.is_empty() {\nLL | |         panic!(\"qaqaq{:?}\", a);\nLL | |     }\n   | |_____^ help: try: `assert!(a.is_empty(), $crate::const_format_args!($fmt, $($arg)+));`\n   |\n   = note: `-D clippy::if-then-panic` implied by `-D warnings`\n\n"}
{"message":"only a `panic!` in `if`-then statement","code":{"code":"clippy::if_then_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":430,"byte_end":479,"line_start":22,"line_end":24,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !a.is_empty() {","highlight_start":5,"highlight_end":23},{"text":"        panic!(\"qwqwq\");","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":430,"byte_end":479,"line_start":22,"line_end":24,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !a.is_empty() {","highlight_start":5,"highlight_end":23},{"text":"        panic!(\"qwqwq\");","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"assert!(a.is_empty(), \"qwqwq\");","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: only a `panic!` in `if`-then statement\n  --> tests/ui/if_then_panic.rs:22:5\n   |\nLL | /     if !a.is_empty() {\nLL | |         panic!(\"qwqwq\");\nLL | |     }\n   | |_____^ help: try: `assert!(a.is_empty(), \"qwqwq\");`\n\n"}
{"message":"only a `panic!` in `if`-then statement","code":{"code":"clippy::if_then_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":765,"byte_end":814,"line_start":39,"line_end":41,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if b.is_empty() {","highlight_start":5,"highlight_end":22},{"text":"        panic!(\"panic1\");","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":765,"byte_end":814,"line_start":39,"line_end":41,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if b.is_empty() {","highlight_start":5,"highlight_end":22},{"text":"        panic!(\"panic1\");","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"assert!(!b.is_empty(), \"panic1\");","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: only a `panic!` in `if`-then statement\n  --> tests/ui/if_then_panic.rs:39:5\n   |\nLL | /     if b.is_empty() {\nLL | |         panic!(\"panic1\");\nLL | |     }\n   | |_____^ help: try: `assert!(!b.is_empty(), \"panic1\");`\n\n"}
{"message":"only a `panic!` in `if`-then statement","code":{"code":"clippy::if_then_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":819,"byte_end":884,"line_start":42,"line_end":44,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if b.is_empty() && a.is_empty() {","highlight_start":5,"highlight_end":38},{"text":"        panic!(\"panic2\");","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":819,"byte_end":884,"line_start":42,"line_end":44,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if b.is_empty() && a.is_empty() {","highlight_start":5,"highlight_end":38},{"text":"        panic!(\"panic2\");","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"assert!(!(b.is_empty() && a.is_empty()), \"panic2\");","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: only a `panic!` in `if`-then statement\n  --> tests/ui/if_then_panic.rs:42:5\n   |\nLL | /     if b.is_empty() && a.is_empty() {\nLL | |         panic!(\"panic2\");\nLL | |     }\n   | |_____^ help: try: `assert!(!(b.is_empty() && a.is_empty()), \"panic2\");`\n\n"}
{"message":"only a `panic!` in `if`-then statement","code":{"code":"clippy::if_then_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":889,"byte_end":955,"line_start":45,"line_end":47,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a.is_empty() && !b.is_empty() {","highlight_start":5,"highlight_end":39},{"text":"        panic!(\"panic3\");","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":889,"byte_end":955,"line_start":45,"line_end":47,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a.is_empty() && !b.is_empty() {","highlight_start":5,"highlight_end":39},{"text":"        panic!(\"panic3\");","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"assert!(!(a.is_empty() && !b.is_empty()), \"panic3\");","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: only a `panic!` in `if`-then statement\n  --> tests/ui/if_then_panic.rs:45:5\n   |\nLL | /     if a.is_empty() && !b.is_empty() {\nLL | |         panic!(\"panic3\");\nLL | |     }\n   | |_____^ help: try: `assert!(!(a.is_empty() && !b.is_empty()), \"panic3\");`\n\n"}
{"message":"only a `panic!` in `if`-then statement","code":{"code":"clippy::if_then_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":960,"byte_end":1025,"line_start":48,"line_end":50,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if b.is_empty() || a.is_empty() {","highlight_start":5,"highlight_end":38},{"text":"        panic!(\"panic4\");","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":960,"byte_end":1025,"line_start":48,"line_end":50,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if b.is_empty() || a.is_empty() {","highlight_start":5,"highlight_end":38},{"text":"        panic!(\"panic4\");","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"assert!(!(b.is_empty() || a.is_empty()), \"panic4\");","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: only a `panic!` in `if`-then statement\n  --> tests/ui/if_then_panic.rs:48:5\n   |\nLL | /     if b.is_empty() || a.is_empty() {\nLL | |         panic!(\"panic4\");\nLL | |     }\n   | |_____^ help: try: `assert!(!(b.is_empty() || a.is_empty()), \"panic4\");`\n\n"}
{"message":"only a `panic!` in `if`-then statement","code":{"code":"clippy::if_then_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":1030,"byte_end":1096,"line_start":51,"line_end":53,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a.is_empty() || !b.is_empty() {","highlight_start":5,"highlight_end":39},{"text":"        panic!(\"panic5\");","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":1030,"byte_end":1096,"line_start":51,"line_end":53,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a.is_empty() || !b.is_empty() {","highlight_start":5,"highlight_end":39},{"text":"        panic!(\"panic5\");","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"assert!(!(a.is_empty() || !b.is_empty()), \"panic5\");","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: only a `panic!` in `if`-then statement\n  --> tests/ui/if_then_panic.rs:51:5\n   |\nLL | /     if a.is_empty() || !b.is_empty() {\nLL | |         panic!(\"panic5\");\nLL | |     }\n   | |_____^ help: try: `assert!(!(a.is_empty() || !b.is_empty()), \"panic5\");`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.0/src/lib.rs:105:22
