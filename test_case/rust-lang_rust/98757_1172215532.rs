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
#####################################                                     51.7%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-05-20/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_data_structures/src/stable_hasher.rs at line 1:
+use crate::fingerprint::Fingerprint;
 use crate::sip128::SipHasher128;
 use rustc_index::bit_set;
 use rustc_index::vec;
Diff in /checkout/compiler/rustc_data_structures/src/stable_hasher.rs at line 4:
 use smallvec::SmallVec;
 use std::hash::{BuildHasher, Hash, Hasher};
 use std::mem;
-use crate::fingerprint::Fingerprint;
 #[cfg(test)]
 mod tests;
 mod tests;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_data_structures/src/transitive_relation/tests.rs" "/checkout/compiler/rustc_data_structures/src/stable_set.rs" "/checkout/compiler/rustc_data_structures/src/stable_hasher.rs" "/checkout/compiler/rustc_data_structures/src/jobserver.rs" "/checkout/compiler/rustc_data_structures/src/sip128.rs" "/checkout/compiler/rustc_data_structures/src/temp_dir.rs" "/checkout/compiler/rustc_data_structures/src/sorted_map/tests.rs" "/checkout/compiler/rustc_data_structures/src/small_str/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
