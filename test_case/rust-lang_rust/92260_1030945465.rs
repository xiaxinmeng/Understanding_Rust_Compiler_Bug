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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/config.rs at line 676:
 
         set(&mut config.out, build.build_dir.map(String::into));
         t!(fs::create_dir_all(&config.out));
-        config.out = t!(config.out.canonicalize(), format!("failed to canonicalize {}", config.out.display()));
+        config.out = t!(
+            config.out.canonicalize(),
+            format!("failed to canonicalize {}", config.out.display())
 
 
         if config.dry_run {
             let dir = config.out.join("tmp-dry-run");
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/sanity.rs" "/checkout/src/bootstrap/tarball.rs" "/checkout/src/bootstrap/flags.rs" "/checkout/src/bootstrap/job.rs" "/checkout/src/bootstrap/check.rs" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/bootstrap/config.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
