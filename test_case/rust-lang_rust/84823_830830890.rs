plain
   Compiling semver-parser v0.10.2
   Compiling semver v0.11.0
   Compiling cargo_metadata v0.12.0
   Compiling clippy_lints v0.1.53 (/checkout/src/tools/clippy/clippy_lints)
error: trivial cast: `&'static str` as `*const str`
    |
    |
415 |                         sess.struct_err(&format!("error finding Clippy's configuration file: {}", error))
    |
    |
    = note: `-D trivial-casts` implied by `-D warnings`
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
437 |                     "error reading Clippy's configuration file `{}`: {}",
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
     |
     |
1063 |             sess.err(&format!("error reading Clippy's configuration file. `{}` is not a valid Rust version", s));
     |
     |
     = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
   |
   |
56 |     println!("if_chain! {{");
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
   |
   |
60 |     println!("    then {{");
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
   |
   |
61 |     println!("        // report your lint here");
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
   |
   |
62 |     println!("    }}");
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
   |
63 |     println!("}}");
   |              ^^^^
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
179 |                 "    if matches!({}, QPath::LangItem(LangItem::{:?}, _));",
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
183 |             print!("    if match_qpath({}, &[", self.current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
185 |             println!("]);");
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
203 |         print!("    if let ExprKind::");
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
204 |         let current = format!("{}.kind", self.current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
208 |                 println!("Box(ref {}) = {};", inner_pat, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
214 |                 println!("Array(ref {}) = {};", elements_pat, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
215 |                 println!("    if {}.len() == {};", elements_pat, elements.len());
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
217 |                     self.current = format!("{}[{}]", elements_pat, i);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
224 |                 println!("Call(ref {}, ref {}) = {};", func_pat, args_pat, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
227 |                 println!("    if {}.len() == {};", args_pat, args.len());
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
229 |                     self.current = format!("{}[{}]", args_pat, i);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
235 |                     "MethodCall(ref method_name, ref generics, ref args, ref fn_span) = {};",
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
238 |                 println!("    // unimplemented: `ExprKind::MethodCall` is not further destructured at the moment");
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
242 |                 println!("Tup(ref {}) = {};", elements_pat, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
243 |                 println!("    if {}.len() == {};", elements_pat, elements.len());
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
245 |                     self.current = format!("{}[{}]", elements_pat, i);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
254 |                     "Binary(ref {}, ref {}, ref {}) = {};",
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
257 |                 println!("    if BinOpKind::{:?} == {}.node;", op.node, op_pat);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
265 |                 println!("Unary(UnOp::{:?}, ref {}) = {};", op, inner_pat, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
271 |                 println!("Lit(ref {}) = {};", lit_pat, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
273 |                     LitKind::Bool(val) => println!("    if let LitKind::Bool({:?}) = {}.node;", val, lit_pat),
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
274 |                     LitKind::Char(c) => println!("    if let LitKind::Char({:?}) = {}.node;", c, lit_pat),
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
275 |                     LitKind::Err(val) => println!("    if let LitKind::Err({}) = {}.node;", val, lit_pat),
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
276 |                     LitKind::Byte(b) => println!("    if let LitKind::Byte({}) = {}.node;", b, lit_pat),
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
278 |                     LitKind::Int(i, _) => println!("    if let LitKind::Int({}, _) = {}.node;", i, lit_pat),
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
280 |                         "    if let LitKind::Float(_, LitFloatType::Suffixed(_)) = {}.node;",
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
284 |                         "    if let LitKind::Float(_, LitFloatType::Unsuffixed) = {}.node;",
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
289 |                         println!("    if let LitKind::ByteStr(ref {}) = {}.node;", vec_pat, lit_pat);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
290 |                         println!("    if let [{:?}] = **{};", vec, vec_pat);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
294 |                         println!("    if let LitKind::Str(ref {}, _) = {}.node;", str_pat, lit_pat);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
295 |                         println!("    if {}.as_str() == {:?}", str_pat, &*text.as_str())
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
304 |                 println!("Cast(ref {}, ref {}) = {};", cast_pat, cast_ty, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
306 |                     println!("    if let TyKind::Path(ref {}) = {}.kind;", qp_label, cast_ty);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
315 |                 println!("Type(ref {}, _) = {};", cast_pat, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
323 |                 println!("Loop(ref {}, ref {}, {}) = {};", body_pat, label_pat, des, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
333 |                         "If(ref {}, ref {}, Some(ref {})) = {};",
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
339 |                     println!("If(ref {}, ref {}, None) = {};", cond_pat, then_pat, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
350 |                 println!("Match(ref {}, ref {}, {}) = {};", expr_pat, arms_pat, des, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
353 |                 println!("    if {}.len() == {};", arms_pat, arms.len());
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
355 |                     self.current = format!("{}[{}].body", arms_pat, i);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
359 |                         println!("    if let Some(ref {}) = {}[{}].guard;", guard_pat, arms_pat, i);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
363 | ...                   println!("    if let Guard::If(ref {}) = {};", if_expr_pat, guard_pat);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
371 | ...                   "    if let Guard::IfLet(ref {}, ref {}) = {};",
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
381 |                     self.current = format!("{}[{}].pat", arms_pat, i);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
386 |                 println!("Closure(ref capture_clause, ref func, _, _, _) = {};", current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
387 |                 println!("    // unimplemented: `ExprKind::Closure` is not further destructured at the moment");
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
391 |                 println!("Yield(ref sub) = {};", current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
397 |                 println!("Block(ref {}) = {};", block_pat, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
405 |                     "Assign(ref {}, ref {}, ref _span) = {};",
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
418 |                     "AssignOp(ref {}, ref {}, ref {}) = {};",
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
421 |                 println!("    if BinOpKind::{:?} == {}.node;", op.node, op_pat);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
430 |                 println!("Field(ref {}, ref {}) = {};", obj_pat, field_name_pat, current);
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'static str` as `*const str`
    |
    |
431 |                 println!("    if {}.as_str() == {:?}", field_name_pat, field_ident.as_str());
    |
    |
    = help: cast can be replaced by coercion; this might require a temporary variable
---
 finished in 11.614 seconds
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/llvm-finished-building`
Dist llvm-tools-nightly-x86_64-unknown-linux-gnu
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1129:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:18:21
