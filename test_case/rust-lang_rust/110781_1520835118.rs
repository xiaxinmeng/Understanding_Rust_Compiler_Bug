diff
--- a/mir_dump/scratch.main-{closure#0}.005-015.DeadStoreElimination.before.mir
+++ b/mir_dump/scratch.main-{closure#0}.005-015.DeadStoreElimination.after.mir
@@ -1,4 +1,4 @@
-// MIR for `main::{closure#0}` before DeadStoreElimination
+// MIR for `main::{closure#0}` after DeadStoreElimination
 
 fn main::{closure#0}(_1: &[closure@src/main.rs:4:13: 4:15]) -> () {
     debug t => (*((*_1).0: &(std::string::String, std::string::String))); // in scope 0 at src/main.rs:2:9: 2:10
@@ -9,7 +9,6 @@ fn main::{closure#0}(_1: &[closure@src/main.rs:4:13: 4:15]) -> () {
 
     bb0: {
         Retag([fn entry] _1);            // scope 0 at src/main.rs:4:13: 6:6
-        _2 = deref_copy ((*_1).0: &(std::string::String, std::string::String)); // scope 0 at src/main.rs:5:22: 5:23
         PlaceMention((*_2));             // scope 0 at src/main.rs:5:22: 5:23
         return;                          // scope 0 at src/main.rs:6:6: 6:6
     }
