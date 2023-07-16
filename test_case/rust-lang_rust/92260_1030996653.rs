plain
   Compiling globset v0.4.5
   Compiling ignore v0.4.17
   Compiling toml v0.5.7
    Finished dev [unoptimized] target(s) in 1m 07s
[src/bootstrap/config.rs:685] config.out.canonicalize() = Ok(
    "/checkout/obj/build",
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
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
   Compiling globset v0.4.5
   Compiling ignore v0.4.17
   Compiling toml v0.5.7
    Finished dev [unoptimized] target(s) in 25.24s
[src/bootstrap/config.rs:685] config.out.canonicalize() = Ok(
    "/checkout/obj/build",
Build completed successfully in 0:00:43
bin_root (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bin_root ... ok
bootstrap_binary (bootstrap.RustBuild)
---
DirectMap1G:    55574528 kB
+ python3 ../x.py --stage 2 test src/tools/expand-yaml-anchors
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.18s
[src/bootstrap/config.rs:685] config.out.canonicalize() = Ok(
    "/checkout/obj/build",
Ensuring the YAML anchors in the GitHub Actions config were expanded
Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
---
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.17s
Warning: --all-targets is now on by default and does not need to be passed explicitly.
[src/bootstrap/config.rs:685] config.out.canonicalize() = Ok(
    "/checkout/obj/build",
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
   Compiling cc v1.0.69
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.116
---
Build completed successfully in 0:02:48
+ python3 ../x.py build --stage 0 src/tools/build-manifest
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.22s
[src/bootstrap/config.rs:685] config.out.canonicalize() = Ok(
    "/checkout/obj/build",
Building stage0 tool build-manifest (x86_64-unknown-linux-gnu)
---
Build completed successfully in 0:00:14
+ python3 ../x.py test --stage 0 src/tools/compiletest
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.17s
[src/bootstrap/config.rs:685] config.out.canonicalize() = Ok(
    "/checkout/obj/build",
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.69
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.116
---
Build completed successfully in 0:01:20
+ python3 ../x.py test --stage 2 src/tools/tidy
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.18s
[src/bootstrap/config.rs:685] config.out.canonicalize() = Ok(
    "/checkout/obj/build",
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/config.rs at line 680:
         let build = toml.build.unwrap_or_default();
 
         set(&mut config.out, build.build_dir.map(String::into));
-        t!(fs::create_dir_all(&config.out), format!("failed to create build dir: {}", config.out.display()));
+        t!(
+            fs::create_dir_all(&config.out),
+            format!("failed to create build dir: {}", config.out.display())
         config.out = t!(
         config.out = t!(
             dbg!(config.out.canonicalize()),
             format!("failed to canonicalize {}", config.out.display())
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/doc.rs" "/checkout/src/bootstrap/sanity.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/config.rs" "/checkout/compiler/rustc_codegen_cranelift/build_sysroot/src/lib.rs" "/checkout/compiler/rustc_hir_pretty/src/lib.rs" "/checkout/src/rustdoc-json-types/lib.rs" "/checkout/src/bootstrap/builder.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
