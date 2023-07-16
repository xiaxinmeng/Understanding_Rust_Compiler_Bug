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
Diff in /checkout/src/bootstrap/test.rs at line 953:
                 if !p.ends_with(".goml") {
                     eprintln!("A non-goml file was given: `{}`", path.display());
                     panic!("Cannot run rustdoc-gui tests");
                 }
                 }
                 if let Some(name) = path.file_name().and_then(|f| f.to_str()) {
                     command.arg("--file").arg(name);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/install.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/clean.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/sanity.rs" "/checkout/src/bootstrap/tarball.rs" "/checkout/src/bootstrap/util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
