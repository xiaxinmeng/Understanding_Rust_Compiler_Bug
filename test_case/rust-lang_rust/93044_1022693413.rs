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
Diff in /checkout/library/std/src/io/tests.rs at line 607:
 fn forward_arc() {
     // Tests forwarding of Read, Write, and Seek through an Arc<T> when &T implements the trait.
-    use crate::net::TcpStream;
     use crate::fs::File;
-    use crate::sync::Arc;
     use crate::io::SeekFrom;
     use crate::io::SeekFrom;
+    use crate::net::TcpStream;
+    use crate::sync::Arc;
 
     // This test is wrapped in a closure to make sure it typechecks
     // but we do not run it.
Diff in /checkout/library/std/src/io/tests.rs at line 633:
 fn forward_rc() {
     // Tests forwarding of Read, Write, and Seek through an Arc<T> when &T implements the trait.
-    use crate::net::TcpStream;
     use crate::fs::File;
-    use crate::rc::Rc;
     use crate::io::SeekFrom;
     use crate::io::SeekFrom;
+    use crate::net::TcpStream;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/thread/scoped.rs" "/checkout/library/std/src/thread/local/dynamic_tests.rs" "/checkout/library/std/src/thread/local/tests.rs" "/checkout/library/std/src/thread/tests.rs" "/checkout/library/std/src/num/benches.rs" "/checkout/library/std/src/num/tests.rs" "/checkout/library/std/src/io/tests.rs" "/checkout/library/std/src/thread/mod.rs"` failed.
+    use crate::rc::Rc;
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
     // This test is wrapped in a closure to make sure it typechecks
     // This test is wrapped in a closure to make sure it typechecks
     // but we do not run it.
