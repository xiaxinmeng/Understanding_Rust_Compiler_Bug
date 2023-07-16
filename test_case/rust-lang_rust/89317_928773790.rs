plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs at line 337:
         Ok(match &node.kind {
             // I dont know if handling of these 3 is correct
             &ExprKind::Scope { value, .. } => self.recurse_build(value)?,
-            &ExprKind::PlaceTypeAscription { source, .. } |
-            &ExprKind::ValueTypeAscription { source, .. } => self.recurse_build(source)?,
+            &ExprKind::PlaceTypeAscription { source, .. }
+            | &ExprKind::ValueTypeAscription { source, .. } => self.recurse_build(source)?,
 
             // subtle: associated consts are literals this arm handles
             // `<T as Trait>::ASSOC` as well as `12`
Diff in /checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs at line 345:
             &ExprKind::Literal { literal, .. } => self.nodes.push(Node::Leaf(literal)),
 
-            ExprKind::Call { fun, args,  .. } => {
+            ExprKind::Call { fun, args, .. } => {
                 let fun = self.recurse_build(*fun)?;
 
                 let mut new_args = Vec::<NodeId>::with_capacity(args.len());
Diff in /checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs at line 353:
                 }
                 let new_args = self.tcx.arena.alloc_slice(&new_args);
                 self.nodes.push(Node::FunctionCall(fun, new_args))
+            }
+            }
             &ExprKind::Binary { op, lhs, rhs } if Self::check_binop(op) => {
                 let lhs = self.recurse_build(lhs)?;
                 let rhs = self.recurse_build(rhs)?;
Diff in /checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs at line 362:
             &ExprKind::Unary { op, arg } if Self::check_unop(op) => {
                 let arg = self.recurse_build(arg)?;
                 self.nodes.push(Node::UnaryOp(op, arg))
+            }
             // This is necessary so that the following compiles:
             //
             // 