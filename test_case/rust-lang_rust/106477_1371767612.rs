plain
Successfully built e06f197f6629
Successfully tagged rust-ci:latest
Built container sha256:e06f197f6629fd8e98b6abb4a21a44be8ee428eba1c20130b219383f55a7e664
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs at line 2557:
         debug!(?predicates);
         assert_eq!(predicates.0.parent, None);
         let mut obligations = Vec::with_capacity(predicates.0.predicates.len());
-        for (index, (predicate, span)) in
-            predicates.0.predicates.into_iter().enumerate()
-        {
+        for (index, (predicate, span)) in predicates.0.predicates.into_iter().enumerate() {
             let span = *span;
             let cause = cause.clone().derived_cause(parent_trait_pred, |derived| {
                 ImplDerivedObligation(Box::new(ImplDerivedObligationCause {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/auto_trait.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/vtable.rs" "/checkout/compiler/rustc_trait_selection/src/traits/chalk_fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/solve/overflow.rs" "/checkout/compiler/rustc_trait_selection/src/traits/wf.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
