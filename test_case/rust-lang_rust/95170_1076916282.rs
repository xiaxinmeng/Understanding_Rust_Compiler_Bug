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
Diff in /checkout/src/bootstrap/native.rs at line 210:
     if !builder.config.patch_binaries_for_nix {
         // Use `/etc/os-release` instead of `/etc/NIXOS`.
         // The latter one does not exist on NixOS when using tmpfs as root.
-        const NIX_IDS: &[&str] = &[
-            "ID=nixos", "ID='nixos'", "ID=\"nixos\""
-        ];
+        const NIX_IDS: &[&str] = &["ID=nixos", "ID='nixos'", "ID=\"nixos\""];
         let os_release = match File::open("/etc/os-release") {
             Err(e) if e.kind() == ErrorKind::NotFound => return,
             Err(e) => panic!("failed to access /etc/os-release: {}", e),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/library/alloc/benches/btree/mod.rs" "/checkout/library/alloc/benches/btree/set.rs" "/checkout/library/alloc/tests/boxed.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/library/alloc/benches/btree/map.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
