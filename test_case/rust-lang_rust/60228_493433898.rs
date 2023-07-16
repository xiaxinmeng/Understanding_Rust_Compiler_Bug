diff
diff --git a/src/librustc/ich/impls_syntax.rs b/src/librustc/ich/impls_syntax.rs
index 4e5718cc5ef..1c70b933f86 100644
--- a/src/librustc/ich/impls_syntax.rs
+++ b/src/librustc/ich/impls_syntax.rs
@@ -117,6 +117,7 @@ impl_stable_hash_for!(enum ::rustc_target::spec::abi::Abi {

 impl_stable_hash_for!(struct ::syntax::attr::Deprecation { since, note });
 impl_stable_hash_for!(struct ::syntax::attr::Stability {
+    _unused,
     level,
     feature,
     rustc_depr,
diff --git a/src/librustc/middle/stability.rs b/src/librustc/middle/stability.rs
index abcf164cda6..a98200268ec 100644
--- a/src/librustc/middle/stability.rs
+++ b/src/librustc/middle/stability.rs
@@ -442,6 +442,7 @@ impl<'a, 'tcx> Index<'tcx> {
                     const_stability: None,
                     promotable: false,
                     allow_const_fn_ptr: false,
+                    _unused: false,
                 });
                 annotator.parent_stab = Some(stability);
             }
diff --git a/src/libsyntax/attr/builtin.rs b/src/libsyntax/attr/builtin.rs
index 65ca96afab1..f0e692cb850 100644
--- a/src/libsyntax/attr/builtin.rs
+++ b/src/libsyntax/attr/builtin.rs
@@ -116,6 +116,8 @@ pub struct Stability {
     pub promotable: bool,
     /// whether the function has a `#[rustc_allow_const_fn_ptr]` attribute
     pub allow_const_fn_ptr: bool,
+    /// new unused field for reproducing ICE
+    pub _unused: bool,
 }

 /// The available stability levels.
@@ -361,6 +363,7 @@ fn find_stability_generic<'a, I>(sess: &ParseSess,
                                 const_stability: None,
                                 promotable: false,
                                 allow_const_fn_ptr: false,
+                                _unused: false,
                             })
                         }
                         (None, _, _) => {
@@ -424,6 +427,7 @@ fn find_stability_generic<'a, I>(sess: &ParseSess,
                                 const_stability: None,
                                 promotable: false,
                                 allow_const_fn_ptr: false,
+                                _unused: false,
                             })
                         }
                         (None, _) => {
