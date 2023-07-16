 diff
lunch-box. git diff libstd/
diff --git a/src/libstd/thread.rs b/src/libstd/thread.rs
index 3b758c8..f291e83 100644
--- a/src/libstd/thread.rs
+++ b/src/libstd/thread.rs
@@ -255,6 +255,7 @@ impl Builder {
             joined: false,
             packet: my_packet,
             thread: thread,
+            _marker: [],
         }
     }

@@ -487,6 +488,7 @@ pub struct JoinGuard<'a, T: 'a> {
     thread: Thread,
     joined: bool,
     packet: Packet<T>,
+    _marker: [&'a T; 0], // bind the lifetime `'a`
 }

 #[stable(feature = "rust1", since = "1.0.0")]
