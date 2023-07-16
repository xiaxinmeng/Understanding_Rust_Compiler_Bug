plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:11f435bab7d0fe51e2f33ae0ce77f457216db8a6)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-5jd1h3ni/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 7a0461093f01
Successfully tagged rust-ci:latest
Built container sha256:7a0461093f01876ce0dc908b94556a03b9e8606ec60dc5ce6546ecc9b7f61aa3
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281
upload failed: - to s3://rust-lang-ci-sccache2/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 15.68s
##[endgroup]
fmt check
Diff in /checkout/compiler/rustc_codegen_llvm/src/lib.rs at line 230:
     ) -> Result<CompiledModule, FatalError> {
         back::write::codegen(cgcx, diag_handler, module, config)
     }
-    fn prepare_thin(module: ModuleCodegen<Self::Module>, config: &ModuleConfig) -> (String, Self::ThinBuffer) {
+        module: ModuleCodegen<Self::Module>,
+        module: ModuleCodegen<Self::Module>,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_query_system/src/lib.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata/enums/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/declare.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/mod.rs" "/checkout/compiler/rustc_query_system/src/dep_graph/dep_node.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/ffi.rs" "/checkout/compiler/rustc_query_system/src/dep_graph/graph.rs" "/checkout/compiler/rustc_codegen_llvm/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+        config: &ModuleConfig,
+    ) -> (String, Self::ThinBuffer) {
         back::lto::prepare_thin(module, config)
     }
     }
     fn serialize_module(module: ModuleCodegen<Self::Module>) -> (String, Self::ModuleBuffer) {
Diff in /checkout/compiler/rustc_codegen_llvm/src/back/lto.rs at line 8:
 use rustc_codegen_ssa::back::lto::{LtoModuleCodegen, SerializedModule, ThinModule, ThinShared};
 use rustc_codegen_ssa::back::symbol_export;
 use rustc_codegen_ssa::back::write::{
-    CodegenContext, FatLTOInput, ModuleConfig, TargetMachineFactoryConfig
+    CodegenContext, FatLTOInput, ModuleConfig, TargetMachineFactoryConfig,
 };
 use rustc_codegen_ssa::traits::*;
 use rustc_codegen_ssa::{looks_like_rust_object_file, ModuleCodegen, ModuleKind};
Diff in /checkout/compiler/rustc_codegen_llvm/src/back/lto.rs at line 214:
 }
 
 
-pub(crate) fn prepare_thin(module: ModuleCodegen<ModuleLlvm>, config: &ModuleConfig) -> (String, ThinBuffer) {
+pub(crate) fn prepare_thin(
+    module: ModuleCodegen<ModuleLlvm>,
+    config: &ModuleConfig,
+) -> (String, ThinBuffer) {
     let name = module.name;
     let buffer = ThinBuffer::new(module.module_llvm.llmod(), true, config.split_thin_lto_unit);
     (name, buffer)
