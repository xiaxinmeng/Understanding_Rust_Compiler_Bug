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
Diff in /checkout/compiler/rustc_data_structures/src/fingerprint/tests.rs at line 6:
     let a = Fingerprint::new(0xf6622fb349898b06, 0x70be9377b2f9c610);
     let b = Fingerprint::new(0xa9562bf5a2a5303c, 0x67d9b6c82034f13d);
     let c = Fingerprint::new(0x0d013a27811dbbc3, 0x9a3f7b3d9142ec43);
-    let permutations = [
-        (a, b, c),
-        (a, c, b),
-        (b, a, c),
-        (b, c, a),
-        (c, a, b),
-        (c, b, a),
-    ];
+    let permutations = [(a, b, c), (a, c, b), (b, a, c), (b, c, a), (c, a, b), (c, b, a)];
     let f = a.combine_commutative(b).combine_commutative(c);
     for p in &permutations {
         assert_eq!(f, p.0.combine_commutative(p.1).combine_commutative(p.2));
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_data_structures/src/fingerprint/tests.rs" "/checkout/compiler/rustc_data_structures/src/vec_map/tests.rs" "/checkout/compiler/rustc_data_structures/src/fx.rs" "/checkout/compiler/rustc_data_structures/src/fingerprint.rs" "/checkout/compiler/rustc_data_structures/src/obligation_forest/graphviz.rs" "/checkout/compiler/rustc_data_structures/src/stable_set.rs" "/checkout/compiler/rustc_data_structures/src/obligation_forest/mod.rs" "/checkout/compiler/rustc_data_structures/src/thin_vec/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
