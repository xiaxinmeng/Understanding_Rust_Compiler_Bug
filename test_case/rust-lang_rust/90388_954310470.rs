plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/alloc/tests/str.rs at line 1325:
     assert_eq!(split, ["SheeP", "SharK", "TurtlE", "CaT"]);
 
-
 #[test]
 fn test_split_char_iterator_rinclusive() {
 fn test_split_char_iterator_rinclusive() {
     let data = "\nMäry häd ä little lämb\nLittle lämb\n";
Diff in /checkout/library/alloc/tests/str.rs at line 1334:
     assert_eq!(split, ["\nMäry häd ä little lämb", "\nLittle lämb", "\n"]);
 
     let uppercase_separated = "SheepSharkTurtleCat";
-    let split: Vec<&str> = uppercase_separated
-        .split_rinclusive(char::is_uppercase)
-        .collect();
+    let split: Vec<&str> = uppercase_separated.split_rinclusive(char::is_uppercase).collect();
     assert_eq!(split, ["Sheep", "Shark", "Turtle", "Cat"]);
 
Diff in /checkout/library/alloc/tests/str.rs at line 1372:
Diff in /checkout/library/alloc/tests/str.rs at line 1372:
     assert_eq!(split, ["\n", "\nLittle lämb", "\nMäry häd ä little lämb"]);
 
     let uppercase_separated = "SheepSharkTurtleCat";
-    let split: Vec<&str> = uppercase_separated
-        .split_rinclusive(char::is_uppercase)
-        .rev()
-        .collect();
+    let split: Vec<&str> = uppercase_separated.split_rinclusive(char::is_uppercase).rev().collect();
     assert_eq!(split, ["Cat", "Turtle", "Shark", "Sheep"]);
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/benches/num/dec2flt/mod.rs" "/checkout/library/alloc/tests/str.rs" "/checkout/library/alloc/tests/arc.rs" "/checkout/library/alloc/tests/cow_str.rs" "/checkout/library/alloc/tests/fmt.rs" "/checkout/library/alloc/tests/boxed.rs" "/checkout/library/alloc/tests/string.rs" "/checkout/library/core/benches/num/flt2dec/strategy/dragon.rs"` failed.
