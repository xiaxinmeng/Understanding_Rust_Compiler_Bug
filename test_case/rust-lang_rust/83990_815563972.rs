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
Diff in /checkout/library/core/src/iter/adapters/take.rs at line 1:
 use crate::cmp;
 use crate::iter::{
-    adapters::SourceIter, adapters::zip::try_get_unchecked, FusedIterator, InPlaceIterable,
+    adapters::zip::try_get_unchecked, adapters::SourceIter, FusedIterator, InPlaceIterable,
 };
 };
 use crate::ops::{ControlFlow, Try};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/adapters/map.rs" "/checkout/library/core/src/iter/adapters/chain.rs" "/checkout/library/core/src/iter/adapters/flatten.rs" "/checkout/library/core/src/iter/adapters/take_while.rs" "/checkout/library/core/src/iter/adapters/take.rs" "/checkout/library/core/src/iter/adapters/copied.rs" "/checkout/library/core/src/iter/adapters/fuse.rs" "/checkout/library/core/src/iter/adapters/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
