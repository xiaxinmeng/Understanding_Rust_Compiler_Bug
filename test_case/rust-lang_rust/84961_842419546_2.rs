
 ///  * `from` is the sending (remote) [`ActorId`], as reported by the remote theater by theater-specific means
 ///  * `to` is the receiving (local) [`ActorId`], as requested by the remote theater
Mismatch at tests/source/issue_4675.rs:3:
         Foo {
         Foo {
             name: Name::$s($p),
-        }
+    }
     };
 }
 }
 

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

Mismatch at tests/source/issue-2520.rs:2:
 // rustfmt-format_code_in_doc_comments: true
 
 
 //! 