plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 5:
 use super::super::try_process;
 use super::super::ByRefSized;
 use super::super::TrustedRandomAccessNoCoerce;
-use super::super::{Chunks, Chain, Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, Fuse};
+use super::super::{Chain, Chunks, Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, Fuse};
 use super::super::{FlatMap, Flatten};
 use super::super::{FromIterator, Intersperse, IntersperseWith, Product, Sum, Zip};
 use super::super::{
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/adapters/map_while.rs" "/checkout/library/core/src/iter/traits/iterator.rs" "/checkout/library/core/src/iter/adapters/skip_while.rs" "/checkout/library/core/src/iter/traits/double_ended.rs" "/checkout/library/core/src/iter/traits/marker.rs" "/checkout/library/core/src/iter/adapters/scan.rs" "/checkout/library/core/src/iter/range.rs" "/checkout/library/core/src/iter/traits/collect.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
