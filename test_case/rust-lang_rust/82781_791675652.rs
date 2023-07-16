plain
    Checking cranelift-frontend v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking cranelift-jit v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking cranelift-object v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
warning: the item `TryInto` is imported redundantly
  --> src/debuginfo/emit.rs:78:13
78 |         use std::convert::TryInto;
   |             ^^^^^^^^^^^^^^^^^^^^^
   | 
  ::: /checkout/library/std/src/prelude/v1.rs:26:35
  ::: /checkout/library/std/src/prelude/v1.rs:26:35
   |
26 | pub use crate::convert::{TryFrom, TryInto};
   |                                   ------- the item `TryInto` is already defined here
   = note: `#[warn(unused_imports)]` on by default

warning: field is never read: `update_symbols`
  --> src/archive.rs:33:5
---
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
    Checking cranelift-frontend v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking cranelift-jit v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking cranelift-object v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
warning: the item `TryInto` is imported redundantly
  --> src/debuginfo/emit.rs:78:13
78 |         use std::convert::TryInto;
   |             ^^^^^^^^^^^^^^^^^^^^^
   | 
  ::: /checkout/library/std/src/prelude/v1.rs:26:35
  ::: /checkout/library/std/src/prelude/v1.rs:26:35
   |
26 | pub use crate::convert::{TryFrom, TryInto};
   |                                   ------- the item `TryInto` is already defined here
   = note: `#[warn(unused_imports)]` on by default

warning: unused extern crate
  --> src/lib.rs:15:1
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/prelude/v1.rs at line 24:
 #[stable(feature = "edition_2021_prelude", since = "1.50.0")]
 #[doc(no_inline)]
 pub use crate::convert::{TryFrom, TryInto};
+#[stable(feature = "edition_2021_prelude", since = "1.50.0")]
+#[doc(no_inline)]
+pub use crate::iter::FromIterator;
 #[stable(feature = "rust1", since = "1.0.0")]
 #[doc(no_inline)]
 pub use crate::iter::{DoubleEndedIterator, ExactSizeIterator};
Diff in /checkout/library/std/src/prelude/v1.rs at line 30:
 #[stable(feature = "rust1", since = "1.0.0")]
 #[doc(no_inline)]
 pub use crate::iter::{Extend, IntoIterator, Iterator};
-#[stable(feature = "edition_2021_prelude", since = "1.50.0")]
-#[doc(no_inline)]
-pub use crate::iter::FromIterator;
 #[stable(feature = "rust1", since = "1.0.0")]
 #[doc(no_inline)]
 pub use crate::option::Option::{self, None, Some};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/vxworks/rand.rs" "/checkout/library/std/src/prelude/v1.rs" "/checkout/library/std/src/prelude/mod.rs" "/checkout/library/std/src/sys/vxworks/mod.rs" "/checkout/library/std/src/sys/vxworks/process/process_vxworks.rs" "/checkout/library/std/src/sys/unsupported/pipe.rs" "/checkout/library/std/src/sys/vxworks/process/mod.rs" "/checkout/library/std/src/sys/vxworks/thread_local_dtor.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
