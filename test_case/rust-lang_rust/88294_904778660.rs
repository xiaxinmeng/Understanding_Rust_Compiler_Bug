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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/io/error.rs at line 269:
 
     // ErrorKinds which are primarily categorisations for OS error
     // codes should be added above.
     /// An error returned when an operation could not be completed because an
     /// An error returned when an operation could not be completed because an
     /// "end of file" was reached prematurely.
Diff in /checkout/library/std/src/io/error.rs at line 287:
Diff in /checkout/library/std/src/io/error.rs at line 287:
     // "Unusual" error kinds which do not correspond simply to (sets
     // of) OS error codes, should be added just above this comment.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/io/stdio/tests.rs" "/checkout/library/std/src/io/impls.rs" "/checkout/library/std/src/io/util.rs" "/checkout/library/std/src/io/prelude.rs" "/checkout/library/std/src/io/stdio.rs" "/checkout/library/std/src/io/buffered/mod.rs" "/checkout/library/std/src/io/error.rs" "/checkout/library/std/src/io/buffered/linewriter.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
     // `Other` and `Uncategorised` should remain at the end:
-
     /// A custom error that does not fall under any other I/O error kind.
     ///
     /// This can be used to construct your own [`Error`]s that do not match any
