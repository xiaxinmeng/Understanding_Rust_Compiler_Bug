plain
[01:36:13] failures:
[01:36:13] 
[01:36:13] ---- [ui] ui/needless_bool.rs stdout ----
[01:36:13] normalized stderr:
[01:36:13] error: this if-then-else expression will always return true
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         true
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         true
[01:36:13] LL | |     };
[01:36:13]    | |_____^
[01:36:13]    |
[01:36:13]    = note: `-D clippy::needless-bool` implied by `-D warnings`
[01:36:13] 
[01:36:13] error: this if-then-else expression will always return false
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         false
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         false
[01:36:13] LL | |     };
[01:36:13]    | |_____^
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         true
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         false
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `x`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         false
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         true
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `!x`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x && y {
[01:36:13] LL | |         false
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         true
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `!(x && y)`
[01:36:13] 
[01:36:13] error: this if-then-else expression will always return true
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     };
[01:36:13]    | |_____^
[01:36:13] 
[01:36:13] error: this if-then-else expression will always return false
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     };
[01:36:13]    | |_____^
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `return x`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x && y {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `return x && y`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `return !x`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x && y {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `return !(x && y)`
[01:36:13] 
[01:36:13] error: equality checks against true are unnecessary
[01:36:13]    |
[01:36:13] LL |     if x == true {};
[01:36:13] LL |     if x == true {};
[01:36:13]    |        ^^^^^^^^^ help: try simplifying it as shown: `x`
[01:36:13]    = note: `-D clippy::bool-comparison` implied by `-D warnings`
[01:36:13] 
[01:36:13] error: equality checks against false can be replaced by a negation
[01:36:13]   --> $DIR/needless_bool.rs:131:8
[01:36:13]   --> $DIR/needless_bool.rs:131:8
[01:36:13]    |
[01:36:13] LL |     if x == false {};
[01:36:13]    |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`
[01:36:13] 
[01:36:13] error: equality checks against true are unnecessary
[01:36:13]    |
[01:36:13] LL |     if x == true {};
[01:36:13] LL |     if x == true {};
[01:36:13]    |        ^^^^^^^^^ help: try simplifying it as shown: `x`
[01:36:13] error: equality checks against false can be replaced by a negation
[01:36:13]   --> $DIR/needless_bool.rs:142:8
[01:36:13]    |
[01:36:13] LL |     if x == false {};
[01:36:13] LL |     if x == false {};
[01:36:13]    |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL |       } else if returns_bool() {
[01:36:13]    |  ____________^
[01:36:13] LL | |         false
[01:36:13] LL | |         false
[01:36:13] LL | |     } else {
[01:36:13] LL | |         true
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `!returns_bool()`
[01:36:13] error: aborting due to 16 previous errors
[01:36:13] 
[01:36:13] 
[01:36:13] 
[01:36:13] 
[01:36:13] expected stderr:
[01:36:13] error: this if-then-else expression will always return true
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         true
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         true
[01:36:13] LL | |     };
[01:36:13]    | |_____^
[01:36:13]    |
[01:36:13]    = note: `-D clippy::needless-bool` implied by `-D warnings`
[01:36:13] 
[01:36:13] error: this if-then-else expression will always return false
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         false
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         false
[01:36:13] LL | |     };
[01:36:13]    | |_____^
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         true
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         false
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `x`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         false
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         true
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `!x`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x && y {
[01:36:13] LL | |         false
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         true
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `!(x && y)`
[01:36:13] 
[01:36:13] error: this if-then-else expression will always return true
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     };
[01:36:13]    | |_____^
[01:36:13] 
[01:36:13] error: this if-then-else expression will always return false
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     };
[01:36:13]    | |_____^
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `return x`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x && y {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `return x && y`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `return !x`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL | /     if x && y {
[01:36:13] LL | |         return false;
[01:36:13] LL | |     } else {
[01:36:13] LL | |     } else {
[01:36:13] LL | |         return true;
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `return !(x && y)`
[01:36:13] 
[01:36:13] error: equality checks against true are unnecessary
[01:36:13]    |
[01:36:13] LL |     if x == true {};
[01:36:13] LL |     if x == true {};
[01:36:13]    |        ^^^^^^^^^ help: try simplifying it as shown: `x`
[01:36:13]    = note: `-D clippy::bool-comparison` implied by `-D warnings`
[01:36:13] 
[01:36:13] error: equality checks against false can be replaced by a negation
[01:36:13]   --> $DIR/needless_bool.rs:131:8
[01:36:13]   --> $DIR/needless_bool.rs:131:8
[01:36:13]    |
[01:36:13] LL |     if x == false {};
[01:36:13]    |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`
[01:36:13] 
[01:36:13] error: equality checks against true are unnecessary
[01:36:13]    |
[01:36:13] LL |     if x == true {};
[01:36:13] LL |     if x == true {};
[01:36:13]    |        ^^^^^^^^^ help: try simplifying it as shown: `x`
[01:36:13] error: equality checks against false can be replaced by a negation
[01:36:13]   --> $DIR/needless_bool.rs:142:8
[01:36:13]    |
[01:36:13] LL |     if x == false {};
[01:36:13] LL |     if x == false {};
[01:36:13]    |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`
[01:36:13] 
[01:36:13] error: this if-then-else expression returns a bool literal
[01:36:13]    |
[01:36:13] LL |       } else if returns_bool() {
[01:36:13]    |  ____________^
[01:36:13] LL | |         false
[01:36:13] LL | |         false
[01:36:13] LL | |     } else {
[01:36:13] LL | |         true
[01:36:13] LL | |     };
[01:36:13]    | |_____^ help: you can reduce it to: `{ !returns_bool() }`
[01:36:13] error: aborting due to 16 previous errors
[01:36:13] 
[01:36:13] 
[01:36:13] 
[01:36:13] 
[01:36:13] diff of stderr:
[01:36:13] 
[01:36:13]  error: this if-then-else expression will always return true
[01:36:13]     |
[01:36:13]  LL | /     if x {
[01:36:13]  LL | |         true
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         true
[01:36:13]  LL | |     };
[01:36:13]     | |_____^
[01:36:13]     |
[01:36:13]     = note: `-D clippy::needless-bool` implied by `-D warnings`
[01:36:13]  
[01:36:13]  error: this if-then-else expression will always return false
[01:36:13]     |
[01:36:13]  LL | /     if x {
[01:36:13]  LL | |         false
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         false
[01:36:13]  LL | |     };
[01:36:13]     | |_____^
[01:36:13]  
[01:36:13]  error: this if-then-else expression returns a bool literal
[01:36:13]     |
[01:36:13]  LL | /     if x {
[01:36:13]  LL | |         true
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         false
[01:36:13]  LL | |     };
[01:36:13]     | |_____^ help: you can reduce it to: `x`
[01:36:13]  
[01:36:13]  error: this if-then-else expression returns a bool literal
[01:36:13]     |
[01:36:13]  LL | /     if x {
[01:36:13]  LL | |         false
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         true
[01:36:13]  LL | |     };
[01:36:13]     | |_____^ help: you can reduce it to: `!x`
[01:36:13]  
[01:36:13]  error: this if-then-else expression returns a bool literal
[01:36:13]     |
[01:36:13]  LL | /     if x && y {
[01:36:13]  LL | |         false
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         true
[01:36:13]  LL | |     };
[01:36:13]     | |_____^ help: you can reduce it to: `!(x && y)`
[01:36:13]  
[01:36:13]  error: this if-then-else expression will always return true
[01:36:13]     |
[01:36:13]  LL | /     if x {
[01:36:13]  LL | |         return true;
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         return true;
[01:36:13]  LL | |     };
[01:36:13]     | |_____^
[01:36:13]  
[01:36:13]  error: this if-then-else expression will always return false
[01:36:13]     |
[01:36:13]  LL | /     if x {
[01:36:13]  LL | |         return false;
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         return false;
[01:36:13]  LL | |     };
[01:36:13]     | |_____^
[01:36:13]  
[01:36:13]  error: this if-then-else expression returns a bool literal
[01:36:13]     |
[01:36:13]  LL | /     if x {
[01:36:13]  LL | |         return true;
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         return false;
[01:36:13]  LL | |     };
[01:36:13]     | |_____^ help: you can reduce it to: `return x`
[01:36:13]  
[01:36:13]  error: this if-then-else expression returns a bool literal
[01:36:13]     |
[01:36:13]  LL | /     if x && y {
[01:36:13]  LL | |         return true;
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         return false;
[01:36:13]  LL | |     };
[01:36:13]     | |_____^ help: you can reduce it to: `return x && y`
[01:36:13]  
[01:36:13]  error: this if-then-else expression returns a bool literal
[01:36:13]     |
[01:36:13]  LL | /     if x {
[01:36:13]  LL | |         return false;
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         return true;
[01:36:13]  LL | |     };
[01:36:13]     | |_____^ help: you can reduce it to: `return !x`
[01:36:13]  
[01:36:13]  error: this if-then-else expression returns a bool literal
[01:36:13]     |
[01:36:13]  LL | /     if x && y {
[01:36:13]  LL | |         return false;
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         return true;
[01:36:13]  LL | |     };
[01:36:13]     | |_____^ help: you can reduce it to: `return !(x && y)`
[01:36:13]  
[01:36:13]  error: equality checks against true are unnecessary
[01:36:13]     |
[01:36:13]  LL |     if x == true {};
[01:36:13]  LL |     if x == true {};
[01:36:13]     |        ^^^^^^^^^ help: try simplifying it as shown: `x`
[01:36:13]     = note: `-D clippy::bool-comparison` implied by `-D warnings`
[01:36:13]  
[01:36:13]  error: equality checks against false can be replaced by a negation
[01:36:13]    --> $DIR/needless_bool.rs:131:8
[01:36:13]    --> $DIR/needless_bool.rs:131:8
[01:36:13]     |
[01:36:13]  LL |     if x == false {};
[01:36:13]     |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`
[01:36:13]  
[01:36:13]  error: equality checks against true are unnecessary
[01:36:13]     |
[01:36:13]  LL |     if x == true {};
[01:36:13]  LL |     if x == true {};
[01:36:13]     |        ^^^^^^^^^ help: try simplifying it as shown: `x`
[01:36:13]  error: equality checks against false can be replaced by a negation
[01:36:13]    --> $DIR/needless_bool.rs:142:8
[01:36:13]     |
[01:36:13]  LL |     if x == false {};
[01:36:13]  LL |     if x == false {};
[01:36:13]     |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`
[01:36:13]  
[01:36:13]  error: this if-then-else expression returns a bool literal
[01:36:13]     |
[01:36:13]  LL |       } else if returns_bool() {
[01:36:13]     |  ____________^
[01:36:13]  LL | |         false
[01:36:13]  LL | |         false
[01:36:13]  LL | |     } else {
[01:36:13]  LL | |         true
[01:36:13]  LL | |     };
[01:36:13] -   | |_____^ help: you can reduce it to: `{ !returns_bool() }`
[01:36:13] +   | |_____^ help: you can reduce it to: `!returns_bool()`
[01:36:13]  error: aborting due to 16 previous errors
[01:36:13]  
[01:36:13]  
[01:36:13] 
[01:36:13] 
[01:36:13] The actual stderr differed from the expected stderr.
[01:36:13] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-77007bd62c0dadf4/out/test_build_base/needless_bool.stderr
[01:36:13] To update references, run this command from build directory:
[01:36:13] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-77007bd62c0dadf4/out/test_build_base' 'needless_bool.rs'
[01:36:13] 
[01:36:13] error: 1 errors occurred comparing output.
[01:36:13] status: exit code: 1
[01:36:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/needless_bool.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-77007bd62c0dadf4/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-77007bd62c0dadf4/out/test_build_base/needless_bool.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-2ef066c730b0470a.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-4eadf8d363e6401f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-75cd9b5eac785169.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-77007bd62c0dadf4/out/test_build_base/needless_bool.stage-id.aux" "-A" "unused"
[01:36:13] ------------------------------------------
[01:36:13] 
[01:36:13] ------------------------------------------
[01:36:13] stderr:
[01:36:13] stderr:
[01:36:13] ------------------------------------------
[01:36:13] {"message":"this if-then-else expression will always return true","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":685,"byte_end":736,"line_start":31,"line_end":35,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-bool` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression will always return true\n  --> tests/ui/needless_bool.rs:31:5\n   |\nLL | /     if x {\nLL | |         true\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^\n   |\n   = note: `-D clippy::needless-bool` implied by `-D warnings`\n\n"}
[01:36:13] {"message":"this if-then-else expression will always return false","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":742,"byte_end":795,"line_start":36,"line_end":40,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this if-then-else expression will always return false\n  --> tests/ui/needless_bool.rs:36:5\n   |\nLL | /     if x {\nLL | |         false\nLL | |     } else {\nLL | |         false\nLL | |     };\n   | |_____^\n\n"}
[01:36:13] {"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":801,"byte_end":853,"line_start":41,"line_end":45,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":801,"byte_end":853,"line_start":41,"line_end":45,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool.rs:41:5\n   |\nLL | /     if x {\nLL | |         true\nLL | |     } else {\nLL | |         false\nLL | |     };\n   | |_____^ help: you can reduce it to: `x`\n\n"}
[01:36:13] {"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":859,"byte_end":911,"line_start":46,"line_end":50,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":859,"byte_end":911,"line_start":46,"line_end":50,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"!x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool.rs:46:5\n   |\nLL | /     if x {\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `!x`\n\n"}
[01:36:13] {"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":917,"byte_end":974,"line_start":51,"line_end":55,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x && y {","highlight_start":5,"highlight_end":16},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":917,"byte_end":974,"line_start":51,"line_end":55,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x && y {","highlight_start":5,"highlight_end":16},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"!(x && y)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool.rs:51:5\n   |\nLL | /     if x && y {\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `!(x && y)`\n\n"}
[01:36:13] {"message":"this if-then-else expression will always return true","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":1370,"byte_end":1437,"line_start":74,"line_end":78,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this if-then-else expression will always return true\n  --> tests/ui/needless_bool.rs:74:5\n   |\nLL | /     if x {\nLL | |         return true;\nLL | |     } else {\nLL | |         return true;\nLL | |     };\n   | |_____^\n\n"}
[01:36:13] {"message":"this if-then-else expression will always return false","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":1539,"byte_end":1608,"line_start":83,"line_end":87,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this if-then-else expression will always return false\n  --> tests/ui/needless_bool.rs:83:5\n   |\nLL | /     if x {\nLL | |         return false;\nLL | |     } else {\nLL | |         return false;\nLL | |     };\n   | |_____^\n\n"}
[01:36:13] {"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":1683,"byte_end":1751,"line_start":92,"line_end":96,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":1683,"byte_end":1751,"line_start":92,"line_end":96,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"return x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool.rs:92:5\n   |\nLL | /     if x {\nLL | |         return true;\nLL | |     } else {\nLL | |         return false;\nLL | |     };\n   | |_____^ help: you can reduce it to: `return x`\n\n"}
[01:36:13] {"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":1835,"byte_end":1908,"line_start":101,"line_end":105,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x && y {","highlight_start":5,"highlight_end":16},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":1835,"byte_end":1908,"line_start":101,"line_end":105,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x && y {","highlight_start":5,"highlight_end":16},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"return x && y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool.rs:101:5\n   |\nLL | /     if x && y {\nLL | |         return true;\nLL | |     } else {\nLL | |         return false;\nLL | |     };\n   | |_____^ help: you can reduce it to: `return x && y`\n\n"}
[01:36:13] {"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":1983,"byte_end":2051,"line_start":110,"line_end":114,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":1983,"byte_end":2051,"line_start":110,"line_end":114,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"return !x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool.rs:110:5\n   |\nLL | /     if x {\nLL | |         return false;\nLL | |     } else {\nLL | |         return true;\nLL | |     };\n   | |_____^ help: you can reduce it to: `return !x`\n\n"}
[01:36:13] {"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2135,"byte_end":2208,"line_start":119,"line_end":123,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x && y {","highlight_start":5,"highlight_end":16},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2135,"byte_end":2208,"line_start":119,"line_end":123,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x && y {","highlight_start":5,"highlight_end":16},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"return !(x && y)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool.rs:119:5\n   |\nLL | /     if x && y {\nLL | |         return false;\nLL | |     } else {\nLL | |         return true;\nLL | |     };\n   | |_____^ help: you can reduce it to: `return !(x && y)`\n\n"}
[01:36:13] {"message":"equality checks against true are unnecessary","code":{"code":"clippy::bool_comparison","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2248,"byte_end":2257,"line_start":127,"line_end":127,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    if x == true {};","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::bool-comparison` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try simplifying it as shown","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2248,"byte_end":2257,"line_start":127,"line_end":127,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    if x == true {};","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: equality checks against true are unnecessary\n  --> tests/ui/needless_bool.rs:127:8\n   |\nLL |     if x == true {};\n   |        ^^^^^^^^^ help: try simplifying it as shown: `x`\n   |\n   = note: `-D clippy::bool-comparison` implied by `-D warnings`\n\n"}
[01:36:13] {"message":"equality checks against false can be replaced by a negation","code":{"code":"clippy::bool_comparison","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2301,"byte_end":2311,"line_start":131,"line_end":131,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"    if x == false {};","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try simplifying it as shown","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2301,"byte_end":2311,"line_start":131,"line_end":131,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"    if x == false {};","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":"!x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: equality checks against false can be replaced by a negation\n  --> tests/ui/needless_bool.rs:131:8\n   |\nLL |     if x == false {};\n   |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`\n\n"}
[01:36:13] {"message":"equality checks against true are unnecessary","code":{"code":"clippy::bool_comparison","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2493,"byte_end":2502,"line_start":141,"line_end":141,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    if x == true {};","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try simplifying it as shown","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2493,"byte_end":2502,"line_start":141,"line_end":141,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    if x == true {};","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: equality checks against true are unnecessary\n  --> tests/ui/needless_bool.rs:141:8\n   |\nLL |     if x == true {};\n   |        ^^^^^^^^^ help: try simplifying it as shown: `x`\n\n"}
[01:36:13] {"message":"equality checks against false can be replaced by a negation","code":{"code":"clippy::bool_comparison","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2514,"byte_end":2524,"line_start":142,"line_end":142,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"    if x == false {};","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try simplifying it as shown","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2514,"byte_end":2524,"line_start":142,"line_end":142,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"    if x == false {};","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":"!x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: equality checks against false can be replaced by a negation\n  --> tests/ui/needless_bool.rs:142:8\n   |\nLL |     if x == false {};\n   |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`\n\n"}
[01:36:13] {"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2720,"byte_end":2785,"line_start":151,"line_end":155,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else if returns_bool() {","highlight_start":12,"highlight_end":31},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool.rs","byte_start":2720,"byte_end":2785,"line_start":151,"line_end":155,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else if returns_bool() {","highlight_start":12,"highlight_end":31},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"!returns_bool()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool.rs:151:12\n   |\nLL |       } else if returns_bool() {\n   |  ____________^\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `!returns_bool()`\n\n"}
[01:36:13] 
[01:36:13] ------------------------------------------
[01:36:13] 
---
[02:01:52] Verifying status of rustfmt...
[02:01:52] Verifying status of clippy-driver...
[02:01:52] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[02:01:52] 
[02:01:52] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[02:01:52] 
[02:01:52] If you do intend to update 'clippy-driver', please check the error messages above and
[02:01:52] commit another update.
[02:01:52] 
[02:01:52] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[02:01:52] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[02:01:52] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:13014510
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 23 12:23:30 UTC 2019
---
travis_time:end:0b23a748:start=1558614212254768175,finish=1558614212276259331,duration=21491156
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02f0350e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f5aaa05
travis_time:start:0f5aaa05
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1dc16b9e
$ dmesg | grep -i kill
