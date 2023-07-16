plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/tools/build-manifest/src/main.rs at line 285:
     fn add_packages_to(&mut self, manifest: &mut Manifest) {
         for pkg in PkgType::all() {
             let fallback = if pkg.use_docs_fallback() { DOCS_FALLBACK } else { &[] };
-            self.package(
-                pkg,
-                &mut manifest.pkg,
-            );
-            );
+            self.package(pkg, &mut manifest.pkg, fallback);
     }
 
Diff in /checkout/src/tools/build-manifest/src/main.rs at line 325:
 
 
         let mut default = minimal;
         default.extend([HtmlDocs, Rustfmt, Clippy]);
-            "default",
-            &default
-        );
-        );
+        profile("default", &default);
 
         // NOTE: this profile is effectively deprecated; do not add new components to it.
         let mut complete = default;
Diff in /checkout/src/tools/build-manifest/src/main.rs at line 335:
         complete.extend([Rls, RustAnalyzer, RustSrc, LlvmTools, RustAnalysis, Miri]);
-            "complete",
-            &complete
-        );
-        );
+        profile("complete", &complete);
 
         // The compiler libraries are not stable for end users, and they're also huge, so we only
         // `rustc-dev` for nightly users, and only in the "complete" profile. It's still possible
Diff in /checkout/src/tools/build-manifest/src/main.rs at line 401:
                 PkgType::RustStd => {
                     components.push(host_component(pkg));
                     extensions.extend(
-                        TARGETS.iter().filter(|&&target| target != host).map(|target| {
-                            Component::from_pkg(pkg, target)
+                        TARGETS
+                            .iter()
+                            .iter()
+                            .filter(|&&target| target != host)
+                            .map(|target| Component::from_pkg(pkg, target)),
                 }
                 }
                 // so is rust-mingw if it's available for the target
Diff in /checkout/src/tools/build-manifest/src/main.rs at line 425:
                     extensions.push(host_component(pkg));
                 }
                 PkgType::RustcDev | PkgType::RustcDocs => {
-                    extensions.extend(
-                        HOSTS.iter().map(|target| {
-                            Component::from_pkg(pkg, target)
-                    );
-                    );
+                    extensions.extend(HOSTS.iter().map(|target| Component::from_pkg(pkg, target)));
                 }
                 PkgType::RustSrc => {
                     extensions.push(Component::from_pkg(pkg, "*"));
Diff in /checkout/src/tools/build-manifest/src/main.rs at line 467:
         dst: &mut BTreeMap<String, Vec<String>>,
         pkgs: &[PkgType],
     ) {
-        dst.insert(profile_name.to_owned(), pkgs.iter().map(|s| s.manifest_component_name()).collect());
+            profile_name.to_owned(),
+            profile_name.to_owned(),
+            pkgs.iter().map(|s| s.manifest_component_name()).collect(),
     }
 
     fn extend_profile(
Diff in /checkout/src/tools/build-manifest/src/main.rs at line 493:
Diff in /checkout/src/tools/build-manifest/src/main.rs at line 493:
             return;
         }
 
-        let version_info = self
-            .versions
-            .version(&pkg)
-            .expect("failed to load package version");
+        let version_info = self.versions.version(&pkg).expect("failed to load package version");
         let mut is_present = version_info.present;
 
         // Never ship nightly-only components for other trains.
Diff in /checkout/src/tools/build-manifest/src/main.rs at line 535:
             Target::unavailable()
 
 
-        let targets = pkg.targets()
+        let targets = pkg
+            .targets()
             .iter()
             .map(|name| {
                 let target = if is_present {
Diff in /checkout/src/tools/build-manifest/src/manifest.rs at line 1:
 use crate::versions::PkgType;
+use crate::Builder;
+use crate::Builder;
 use serde::{Serialize, Serializer};
 use std::collections::BTreeMap;
 use std::path::{Path, PathBuf};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/lint-docs/src/lib.rs" "/checkout/src/tools/replace-version-placeholder/src/main.rs" "/checkout/src/tools/tier-check/src/main.rs" "/checkout/src/tools/build-manifest/src/main.rs" "/checkout/src/tools/build-manifest/src/checksum.rs" "/checkout/src/tools/build-manifest/src/versions.rs" "/checkout/src/tools/build-manifest/src/manifest.rs" "/checkout/src/tools/lint-docs/src/main.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
