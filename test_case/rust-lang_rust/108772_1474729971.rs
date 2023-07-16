plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (mingw-check-tidy, true, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-shq_f2fc/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built e03e87d8b803
Successfully tagged rust-ci:latest
Built container sha256:e03e87d8b80390fd175be81c06bcb261776eb4fafd17521b52ea6678ee9c0c52
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.14.0
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 16.62s
fmt check
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 876:
                 } => {
                     self.local_decls[local].mutability = mutability;
                     self.local_decls[local].source_info.scope = self.source_scope;
-                    **self.local_decls[local].local_info.as_mut().assert_crate_local() = if let Some(kind) = param.self_kind {
-                        LocalInfo::User(
-                            BindingForm::ImplicitSelf(kind),
-                    } else {
-                    } else {
-                        let binding_mode = ty::BindingMode::BindByValue(mutability);
-                        LocalInfo::User(BindingForm::Var(
-                            VarBindingForm {
+                    **self.local_decls[local].local_info.as_mut().assert_crate_local() =
+                        if let Some(kind) = param.self_kind {
+                            LocalInfo::User(BindingForm::ImplicitSelf(kind))
+                        } else {
+                            let binding_mode = ty::BindingMode::BindByValue(mutability);
+                            LocalInfo::User(BindingForm::Var(VarBindingForm {
                                 opt_ty_info: param.ty_span,
                                 opt_ty_info: param.ty_span,
                                 opt_match_place: Some((None, span)),
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 890:
                                 pat_span: span,
-                        ))
-                    };
+                            }))
+                        };
+                        };
                     self.var_indices.insert(var, LocalsForNode::One(local));
                 _ => {
Build completed unsuccessfully in 0:00:18
Build completed unsuccessfully in 0:00:18
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/mir/coverageinfo.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs" "/checkout/compiler/rustc_mir_build/src/build/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/mono_item.rs" "/checkout/compiler/rustc_mir_build/src/build/scope.rs" "/checkout/compiler/rustc_codegen_ssa/src/lib.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/statement.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/symbol_export.rs"` failed.
Diff in /checkout/compiler/rustc_mir_build/src/build/matches/mod.rs at line 2252:
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                 user_ty: None,
                 internal: false,
                 internal: false,
-                local_info: ClearCrossCrate::Set(Box::new(LocalInfo::User(BindingForm::RefForGuard))),
+                local_info: ClearCrossCrate::Set(Box::new(LocalInfo::User(
+                    BindingForm::RefForGuard,
+                ))),
             });
             self.var_debug_info.push(VarDebugInfo {
                 name,
Diff in /checkout/compiler/rustc_mir_build/src/build/expr/as_rvalue.rs at line 73:
             }
             ExprKind::Binary { op, lhs, rhs } => {
                 let lhs = unpack!(
-                    block =
-                        this.as_operand(block, scope, &this.thir[lhs], LocalInfo::Boring, NeedsTemporary::Maybe)
+                    block = this.as_operand(
+                        scope,
+                        scope,
+                        &this.thir[lhs],
+                        LocalInfo::Boring,
+                        NeedsTemporary::Maybe
                 );
                 let rhs = unpack!(
-                    block =
-                    block =
-                        this.as_operand(block, scope, &this.thir[rhs], LocalInfo::Boring, NeedsTemporary::No)
+                    block = this.as_operand(
+                        scope,
+                        scope,
+                        &this.thir[rhs],
+                        LocalInfo::Boring,
