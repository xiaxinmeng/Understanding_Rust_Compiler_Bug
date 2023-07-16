
 /// # #![cfg_attr(not(dox), feature(cfg_target_feature, target_feature, stdsimd))]
 /// # #![cfg_attr(not(dox), no_std)]
-/// fn foo() {}
+/// fn foo() {  }
 fn foo() {}
 

Mismatch at tests/source/issue_4057.rs:7:
Mismatch at tests/source/issue_4057.rs:7:
 /// # type Projection<'a> = &'a ();
 /// # type ProjectionRef<'a> = &'a ();
 /// # trait Dox {
-/// fn project_ex(self: Pin<&mut Self>) -> Projection<'_>;
-/// fn project_ref(self: Pin<&Self>) -> ProjectionRef<'_>;
+/// fn   project_ex (self: Pin<&mut Self>) -> Projection<'_>;
+/// fn   project_ref(self: Pin<&Self>) -> ProjectionRef<'_>;
 /// # }
 /// # }

Mismatch at tests/source/issue_4675.rs:3:
         Foo {
         Foo {
             name: Name::$s($p),
-        }
+    }
     };
 }
---
 

Mismatch at tests/source/issue-3055/original.rs:32:
 ///
 /// Should format with rust attribute
 /// 