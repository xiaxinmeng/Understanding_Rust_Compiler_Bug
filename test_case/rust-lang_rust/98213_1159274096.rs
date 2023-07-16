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
Diff in /checkout/src/tools/rustbook/src/main.rs at line 3:
 use std::env;
 use std::path::{Path, PathBuf};
 
-use clap::{arg, Command, ArgMatches};
+use clap::{arg, ArgMatches, Command};
 use mdbook::errors::Result as Result3;
 use mdbook::errors::Result as Result3;
 use mdbook::MDBook;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/build-manifest/src/manifest.rs" "/checkout/src/tools/tidy/src/unit_tests.rs" "/checkout/src/tools/build-manifest/src/versions.rs" "/checkout/src/tools/tidy/src/extdeps.rs" "/checkout/src/tools/build-manifest/src/checksum.rs" "/checkout/src/tools/tidy/src/debug_artifacts.rs" "/checkout/src/tools/build-manifest/src/main.rs" "/checkout/src/tools/rustbook/src/main.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
