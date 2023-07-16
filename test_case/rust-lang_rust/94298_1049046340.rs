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
Diff in /checkout/src/bootstrap/builder.rs at line 26:
 use crate::test;
 use crate::tool::{self, SourceType};
 use crate::util::{self, add_dylib_path, add_link_lib_path, exe, libdir};
-use crate::{Build, DocTests, GitRepo, Mode};
 use crate::EXTRA_CHECK_CFGS;
+use crate::{Build, DocTests, GitRepo, Mode};
 pub use crate::Compiler;
 pub use crate::Compiler;
 // FIXME: replace with std::lazy after it gets stabilized and reaches beta
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/unwind/build.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/library/unwind/src/lib.rs" "/checkout/library/unwind/src/libunwind.rs" "/checkout/compiler/rustc_llvm/build.rs" "/checkout/compiler/rustc_mir_transform/src/lower_slice_len.rs" "/checkout/compiler/rustc_attr/src/builtin.rs" "/checkout/library/core/src/future/future.rs"` failed.
Build completed unsuccessfully in 0:00:13
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
