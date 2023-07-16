
diff --git a/src/librustc/middle/ty.rs b/src/librustc/middle/ty.rs
index 43932c5..11cbf3a 100644
--- a/src/librustc/middle/ty.rs
+++ b/src/librustc/middle/ty.rs
@@ -1766,7 +1766,6 @@ def_type_content_sets!(

         // Things that are interior to the value (first nibble):
         InteriorUnsized                     = 0b0000__00000000__0001,
-        InteriorNonpod                      = 0b0000__00000000__0010,
         // InteriorAll                         = 0b0000__00000000__1111,

         // Things that are owned by the value (second and third nibbles):
@@ -1808,7 +1807,7 @@ def_type_content_sets!(
         Nonsized                            = 0b0000__00000000__0001,

         // Things that make values considered not POD
-        Nonpod                              = 0b0000__00001011__0010,
+        Nonpod                              = 0b0000__00001011__0000,

         // Bits to set when a managed value is encountered
         //
@@ -1882,8 +1881,7 @@ impl TypeContents {
          * Includes only those bits that still apply
          * when indirected through a `~` pointer
          */
-        TC::OwnsOwned |
-        TC::InteriorNonpod | (
+        TC::OwnsOwned | (
             *self & (TC::OwnsAll | TC::ReachesAll))
     }

@@ -2014,7 +2012,7 @@ pub fn type_contents(cx: ctxt, ty: t) -> TypeContents {
             }

             ty_estr(vstore_uniq) => {
-                TC::OwnsOwned | TC::InteriorNonpod
+                TC::OwnsOwned
             }

             ty_closure(ref c) => {
@@ -2133,7 +2131,7 @@ pub fn type_contents(cx: ctxt, ty: t) -> TypeContents {
                 match sigil {
                     ast::BorrowedSigil => TC::ReachesBorrowed,
                     ast::ManagedSigil => TC::Managed,
-                    ast::OwnedSigil => TC::OwnsOwned | TC::InteriorNonpod,
+                    ast::OwnedSigil => TC::OwnsOwned,
                 }
             }
