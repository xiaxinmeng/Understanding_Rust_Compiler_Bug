diff
diff --git a/components/script/dom/bindings/interface.rs b/components/script/dom/bindings/interface.rs
index 210e9ae557..7632d341b1 100644
--- a/components/script/dom/bindings/interface.rs
+++ b/components/script/dom/bindings/interface.rs
@@ -88,6 +88,7 @@ pub struct InterfaceConstructorBehavior(ClassOps);
 
 impl InterfaceConstructorBehavior {
     /// An interface constructor that unconditionally throws a type error.
+    #[rustc_promotable]
     pub const fn throw() -> Self {
         InterfaceConstructorBehavior(ClassOps {
             addProperty: None,
@@ -105,6 +106,7 @@ impl InterfaceConstructorBehavior {
     }
 
     /// An interface constructor that calls a native Rust function.
+    #[rustc_promotable]
     pub const fn call(hook: ConstructorClassHook) -> Self {
         InterfaceConstructorBehavior(ClassOps {
             addProperty: None,
diff --git a/components/script/lib.rs b/components/script/lib.rs
index 097f69d780..a5355a3937 100644
--- a/components/script/lib.rs
+++ b/components/script/lib.rs
@@ -7,6 +7,7 @@
 #![feature(const_fn)]
 #![feature(drain_filter)]
 #![feature(plugin)]
+#![feature(rustc_attrs)]
 #![feature(try_from)]
 #![deny(unsafe_code)]
 #![allow(non_snake_case)]
