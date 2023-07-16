diff
diff --git i/library/core/src/ptr/mod.rs w/library/core/src/ptr/mod.rs
index 8d3f4e8f569..74a91394806 100644
--- i/library/core/src/ptr/mod.rs
+++ w/library/core/src/ptr/mod.rs
@@ -1855,10 +1855,10 @@ macro_rules! maybe_fnptr_doc {
 
 // Impls for function pointers
 macro_rules! fnptr_impls_safety_abi {
-    ($FnTy: ty, $($Arg: ident),*) => {
+    (#[$meta:meta] $FnTy: ty, $($Arg: ident),*) => {
         maybe_fnptr_doc! {
             $($Arg)* @
-            #[stable(feature = "fnptr_impls", since = "1.4.0")]
+            #[$meta]
             impl<Ret, $($Arg),*> PartialEq for $FnTy {
                 #[inline]
                 fn eq(&self, other: &Self) -> bool {
@@ -1869,13 +1869,13 @@ fn eq(&self, other: &Self) -> bool {
 
         maybe_fnptr_doc! {
             $($Arg)* @
-            #[stable(feature = "fnptr_impls", since = "1.4.0")]
+            #[$meta]
             impl<Ret, $($Arg),*> Eq for $FnTy {}
         }
 
         maybe_fnptr_doc! {
             $($Arg)* @
-            #[stable(feature = "fnptr_impls", since = "1.4.0")]
+            #[$meta]
             impl<Ret, $($Arg),*> PartialOrd for $FnTy {
                 #[inline]
                 fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
@@ -1886,7 +1886,7 @@ fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
 
         maybe_fnptr_doc! {
             $($Arg)* @
-            #[stable(feature = "fnptr_impls", since = "1.4.0")]
+            #[$meta]
             impl<Ret, $($Arg),*> Ord for $FnTy {
                 #[inline]
                 fn cmp(&self, other: &Self) -> Ordering {
@@ -1897,7 +1897,7 @@ fn cmp(&self, other: &Self) -> Ordering {
 
         maybe_fnptr_doc! {
             $($Arg)* @
-            #[stable(feature = "fnptr_impls", since = "1.4.0")]
+            #[$meta]
             impl<Ret, $($Arg),*> hash::Hash for $FnTy {
                 fn hash<HH: hash::Hasher>(&self, state: &mut HH) {
                     state.write_usize(*self as usize)
@@ -1907,7 +1907,7 @@ fn hash<HH: hash::Hasher>(&self, state: &mut HH) {
 
         maybe_fnptr_doc! {
             $($Arg)* @
-            #[stable(feature = "fnptr_impls", since = "1.4.0")]
+            #[$meta]
             impl<Ret, $($Arg),*> fmt::Pointer for $FnTy {
                 fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                     fmt::pointer_fmt_inner(*self as usize, f)
@@ -1917,7 +1917,7 @@ fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
 
         maybe_fnptr_doc! {
             $($Arg)* @
-            #[stable(feature = "fnptr_impls", since = "1.4.0")]
+            #[$meta]
             impl<Ret, $($Arg),*> fmt::Debug for $FnTy {
                 fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                     fmt::pointer_fmt_inner(*self as usize, f)
@@ -1929,25 +1929,25 @@ fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
 
 macro_rules! fnptr_impls_args {
     ($($Arg: ident),+) => {
-        fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),+) -> Ret, $($Arg),+ }
-        fnptr_impls_safety_abi! { extern "C" fn($($Arg),+) -> Ret, $($Arg),+ }
-        fnptr_impls_safety_abi! { extern "C" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
-        fnptr_impls_safety_abi! { extern "C-unwind" fn($($Arg),+) -> Ret, $($Arg),+ }
-        fnptr_impls_safety_abi! { extern "C-unwind" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
-        fnptr_impls_safety_abi! { unsafe extern "Rust" fn($($Arg),+) -> Ret, $($Arg),+ }
-        fnptr_impls_safety_abi! { unsafe extern "C" fn($($Arg),+) -> Ret, $($Arg),+ }
-        fnptr_impls_safety_abi! { unsafe extern "C" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
-        fnptr_impls_safety_abi! { unsafe extern "C-unwind" fn($($Arg),+) -> Ret, $($Arg),+ }
-        fnptr_impls_safety_abi! { unsafe extern "C-unwind" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] extern "Rust" fn($($Arg),+) -> Ret, $($Arg),+ }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] extern "C" fn($($Arg),+) -> Ret, $($Arg),+ }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] extern "C" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
+        fnptr_impls_safety_abi! { #[unstable(feature = "c_unwind", issue = "74990")] extern "C-unwind" fn($($Arg),+) -> Ret, $($Arg),+ }
+        fnptr_impls_safety_abi! { #[unstable(feature = "c_unwind", issue = "74990")] extern "C-unwind" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] unsafe extern "Rust" fn($($Arg),+) -> Ret, $($Arg),+ }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] unsafe extern "C" fn($($Arg),+) -> Ret, $($Arg),+ }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] unsafe extern "C" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] unsafe extern "C-unwind" fn($($Arg),+) -> Ret, $($Arg),+ }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] unsafe extern "C-unwind" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
     };
     () => {
         // No variadic functions with 0 parameters
-        fnptr_impls_safety_abi! { extern "Rust" fn() -> Ret, }
-        fnptr_impls_safety_abi! { extern "C" fn() -> Ret, }
-        fnptr_impls_safety_abi! { extern "C-unwind" fn() -> Ret, }
-        fnptr_impls_safety_abi! { unsafe extern "Rust" fn() -> Ret, }
-        fnptr_impls_safety_abi! { unsafe extern "C" fn() -> Ret, }
-        fnptr_impls_safety_abi! { unsafe extern "C-unwind" fn() -> Ret, }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] extern "Rust" fn() -> Ret, }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] extern "C" fn() -> Ret, }
+        fnptr_impls_safety_abi! { #[unstable(feature = "c_unwind", issue = "74990")] extern "C-unwind" fn() -> Ret, }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] unsafe extern "Rust" fn() -> Ret, }
+        fnptr_impls_safety_abi! { #[stable(feature = "fnptr_impls", since = "1.4.0")] unsafe extern "C" fn() -> Ret, }
+        fnptr_impls_safety_abi! { #[unstable(feature = "c_unwind", issue = "74990")] unsafe extern "C-unwind" fn() -> Ret, }
     };
 }
 