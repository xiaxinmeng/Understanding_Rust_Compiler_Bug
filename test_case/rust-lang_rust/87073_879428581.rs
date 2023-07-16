plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
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
Diff in /checkout/src/tools/linkchecker/main.rs at line 37:
     ("std/collections/btree_set/struct.BTreeSet.html", &["#insert-and-complex-keys"]),
     ("alloc/collections/btree_map/struct.BTreeMap.html", &["#insert-and-complex-keys"]),
     ("alloc/collections/btree_set/struct.BTreeSet.html", &["#insert-and-complex-keys"]),
-
     // These try to link to various things in std, but are defined in core.
     // The docs in std::primitive use proper intra-doc links, so these seem fine to special-case.
     ("alloc/slice/trait.Join.html", &["core/primitive.slice.html#method.join"]),
Diff in /checkout/src/tools/linkchecker/main.rs at line 44:
     ("alloc/slice/trait.Concat.html", &["core/primitive.slice.html#method.concat"]),
-    ("alloc/slice/index.html", &["core/primitive.slice.html#method.concat", "core/primitive.slice.html#method.concat"]),
-    ("alloc/vec/struct.Vec.html", &["core/primitive.slice.html#method.sort_by_key", "core/primitive.slice.html#method.sort_by_cached_key"]),
+    (
+        "alloc/slice/index.html",
+        &["core/primitive.slice.html#method.concat", "core/primitive.slice.html#method.concat"],
+    (
+    (
+        "alloc/vec/struct.Vec.html",
+        &[
+            "core/primitive.slice.html#method.sort_by_key",
+            "core/primitive.slice.html#method.sort_by_cached_key",
+    ),
+    ),
     // These are broken because liballoc uses `#[lang_item]` magic to define things on primitives that aren't available in core.
     ("core/primitive.str.html", &["#method.to_ascii_uppercase", "#method.to_ascii_lowercase"]),
-    ("core/primitive.slice.html", &["#method.to_ascii_uppercase", "#method.to_ascii_lowercase", "#method.sort_by_cached_key"]),
+    (
+        "core/primitive.slice.html",
+        &["#method.to_ascii_uppercase", "#method.to_ascii_lowercase", "#method.sort_by_cached_key"],
 ];
 
 
 #[rustfmt::skip]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/linkchecker/main.rs" "/checkout/src/tools/cargotest/main.rs" "/checkout/src/tools/linkchecker/tests/checks.rs" "/checkout/src/tools/lint-docs/src/lib.rs" "/checkout/src/tools/lint-docs/src/groups.rs" "/checkout/src/tools/tier-check/src/main.rs" "/checkout/src/tools/compiletest/src/util/tests.rs" "/checkout/src/tools/html-checker/main.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
