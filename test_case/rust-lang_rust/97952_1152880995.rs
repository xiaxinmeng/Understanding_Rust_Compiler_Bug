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
Diff in /checkout/compiler/rustc_data_structures/src/lib.rs at line 77:
 pub mod map_in_place;
 pub mod obligation_forest;
 pub mod owning_ref;
-mod xxhash;
 pub mod small_c_str;
 pub mod small_str;
 pub mod snapshot_map;
Diff in /checkout/compiler/rustc_data_structures/src/lib.rs at line 84:
 pub mod stable_map;
 pub mod svh;
+mod xxhash;
 pub use ena::snapshot_vec;
 pub mod memmap;
 pub mod sorted_map;
Diff in /checkout/compiler/rustc_data_structures/src/stable_hasher.rs at line 1:
+use crate::xxhash::Xxh3Hasher;
 use rustc_index::bit_set;
 use rustc_index::vec;
 use smallvec::SmallVec;
Diff in /checkout/compiler/rustc_data_structures/src/stable_hasher.rs at line 4:
 use std::hash::{BuildHasher, Hash, Hasher};
 use std::mem;
-use crate::xxhash::Xxh3Hasher;
 #[cfg(test)]
 mod tests;
Diff in /checkout/compiler/rustc_data_structures/src/stable_hasher.rs at line 16:
Diff in /checkout/compiler/rustc_data_structures/src/stable_hasher.rs at line 16:
 /// hashing and the architecture dependent `isize` and `usize` types are
 /// extended to 64 bits if needed.
 pub struct StableHasher {
-    state: Xxh3Hasher
+    state: Xxh3Hasher,
 
 impl ::std::fmt::Debug for StableHasher {
 impl ::std::fmt::Debug for StableHasher {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_data_structures/src/temp_dir.rs" "/checkout/compiler/rustc_data_structures/src/sorted_map.rs" "/checkout/compiler/rustc_data_structures/src/transitive_relation.rs" "/checkout/compiler/rustc_data_structures/src/thin_vec/tests.rs" "/checkout/compiler/rustc_data_structures/src/thin_vec.rs" "/checkout/compiler/rustc_data_structures/src/stable_hasher.rs" "/checkout/compiler/rustc_data_structures/src/small_c_str/tests.rs" "/checkout/compiler/rustc_data_structures/src/vec_map.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
