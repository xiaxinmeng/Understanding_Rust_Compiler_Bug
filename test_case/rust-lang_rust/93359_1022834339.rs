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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/io/buffered/bufreader.rs at line 1:
 use crate::cmp;
 use crate::fmt;
 use crate::io::{
-    self, BufRead, IoSliceMut, Read, ReadBuf, ReadBufRef, Seek, SeekFrom, SizeHint, DEFAULT_BUF_SIZE,
+    self, BufRead, IoSliceMut, Read, ReadBuf, ReadBufRef, Seek, SeekFrom, SizeHint,
+    DEFAULT_BUF_SIZE,
 use crate::mem::MaybeUninit;
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/io/impls/tests.rs" "/checkout/library/std/src/io/buffered/linewriter.rs" "/checkout/library/std/src/io/buffered/bufwriter.rs" "/checkout/library/std/src/io/buffered/linewritershim.rs" "/checkout/library/std/src/io/error.rs" "/checkout/library/std/src/io/buffered/tests.rs" "/checkout/library/std/src/io/buffered/bufreader.rs" "/checkout/library/std/src/io/cursor/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
