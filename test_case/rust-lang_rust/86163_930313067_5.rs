
--- rustc-1.53.0-src/compiler/rustc_target/src/abi/call/sparc64.rs
+++ rustc-1.53.0-src/compiler/rustc_target/src/abi/call/sparc64.rs
@@ -1,7 +1,7 @@
 // FIXME: This needs an audit for correctness and completeness.

-use crate::abi::call::{ArgAbi, FnAbi, Reg, RegKind, Uniform};
-use crate::abi::{HasDataLayout, LayoutOf, TyAndLayout, TyAndLayoutMethods};
+use crate::abi::call::{ArgAbi, ArgAttribute, FnAbi, PassMode, Reg, RegKind, Uniform};
+use crate::abi::{self, HasDataLayout, LayoutOf, Size, TyAndLayout, TyAndLayoutMethods};

 fn is_homogeneous_aggregate<'a, Ty, C>(cx: &C, arg: &mut ArgAbi<'a, Ty>) -> Option<Uniform>
 where
@@ -14,6 +14,10 @@
             return None;
         }

+        if unit.kind == RegKind::Float && unit.size == (Size { raw: 4 }) {
+            return None;
+        }
+
         let valid_unit = match unit.kind {
             RegKind::Integer => false,
             RegKind::Float => true,
@@ -65,6 +69,41 @@
         return;
     }

+    match arg.layout.fields {
+        abi::FieldsShape::Primitive |
+        abi::FieldsShape::Array { .. } |
+        abi::FieldsShape::Union(_) => { }
+        abi::FieldsShape::Arbitrary { .. } => {
+
+            let mut has_f32_type = false;
+            let mut has_64b_type = false;
+
+            for i in 0..arg.layout.fields.count() {
+
+                let field = arg.layout.field(cx, i);
+                // We only care about aligned doubles
+                if let abi::Abi::Scalar(scalar) = &field.abi {
+                    if let abi::F32 = scalar.value {
+                        has_f32_type = true;
+                    } else if let abi::F64 = scalar.value {
+                        has_64b_type = true;
+                    } else if let abi::Int(i, _signed) = scalar.value {
+                        has_64b_type = i.size().bits() == 64;
+                    } else if let abi::Pointer = scalar.value {
+                        has_64b_type = true;
+                    }
+                }
+             }
+
+             if has_f32_type && !has_64b_type {
+                 if let PassMode::Direct(ref mut attrs) = arg.mode {
+                     attrs.set(ArgAttribute::InReg);
+                     return;
+                 }
+             }
+        }
+    };
+
     let total = arg.layout.size;
     if total.bits() > 128 {
         arg.make_indirect();
