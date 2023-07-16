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
Diff in /checkout/compiler/rustc_interface/src/queries.rs at line 3:
 
 use rustc_ast as ast;
 use rustc_codegen_ssa::traits::CodegenBackend;
+use rustc_codegen_ssa::CodegenResults;
 use rustc_data_structures::svh::Svh;
 use rustc_data_structures::sync::{Lrc, OnceCell, WorkerLocal};
 use rustc_hir::def_id::LOCAL_CRATE;
Diff in /checkout/compiler/rustc_interface/src/queries.rs at line 18:
 use std::any::Any;
 use std::cell::{Ref, RefCell, RefMut};
 use std::rc::Rc;
-use rustc_codegen_ssa::CodegenResults;
 /// Represent the result of a query.
 ///
Diff in /checkout/compiler/rustc_codegen_ssa/src/lib.rs at line 29:
Diff in /checkout/compiler/rustc_codegen_ssa/src/lib.rs at line 29:
 use rustc_middle::dep_graph::WorkProduct;
 use rustc_middle::middle::dependency_format::Dependencies;
 use rustc_middle::ty::query::{ExternProviders, Providers};
+use rustc_serialize::{opaque, Decodable, Decoder, Encoder};
 use rustc_session::config::{CrateType, OutputFilenames, OutputType, RUST_CGU_EXT};
 use rustc_session::cstore::{self, CrateSource};
 use rustc_session::utils::NativeLibKind;
Diff in /checkout/compiler/rustc_codegen_ssa/src/lib.rs at line 35:
 use rustc_span::symbol::Symbol;
 use std::path::{Path, PathBuf};
-use rustc_serialize::{Decodable, Decoder, Encoder, opaque};
 pub mod back;
 pub mod base;
Diff in /checkout/compiler/rustc_codegen_ssa/src/lib.rs at line 214:
         let rustc_version = decoder.read_str();
         let rustc_version = decoder.read_str();
         let current_version = RUSTC_VERSION.unwrap();
         if rustc_version != current_version {
-            return Err(format!(".rlink file was produced by rustc version {rustc_version}, but the current version is {current_version}."));
+            return Err(format!(
+                ".rlink file was produced by rustc version {rustc_version}, but the current version is {current_version}."
         }
 
         let codegen_results = CodegenResults::decode(&mut decoder);
         let codegen_results = CodegenResults::decode(&mut decoder);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/coverageinfo/ffi.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/lto.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath/tests.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/linker.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/write.rs" "/checkout/compiler/rustc_codegen_ssa/src/coverageinfo/map.rs" "/checkout/compiler/rustc_codegen_ssa/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
