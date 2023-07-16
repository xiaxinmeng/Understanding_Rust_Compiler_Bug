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
Diff in /checkout/compiler/rustc_data_structures/src/stable_hasher.rs at line 563:
     length: usize,
     hash_function: F,
 ) where
-    C: Iterator<Item=I>,
+    C: Iterator<Item = I>,
     F: Fn(&mut StableHasher, &mut HCX, I),
     let hash = collection
     let hash = collection
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_data_structures/src/owning_ref/tests.rs" "/checkout/compiler/rustc_data_structures/src/flock.rs" "/checkout/compiler/rustc_data_structures/src/work_queue.rs" "/checkout/compiler/rustc_data_structures/src/stable_hasher.rs" "/checkout/compiler/rustc_data_structures/src/vec_linked_list.rs" "/checkout/compiler/rustc_data_structures/src/transitive_relation/tests.rs" "/checkout/compiler/rustc_data_structures/src/atomic_ref.rs" "/checkout/compiler/rustc_data_structures/src/owning_ref/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
