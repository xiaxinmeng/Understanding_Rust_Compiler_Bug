
---- [ui] ui/issues/issue-27033.rs stdout ----
diff of stderr:

3	   |
4	LL |         None @ _ => {}
5	   |         ^^^^ cannot be named the same as a unit variant
-	   | 
-	  ::: $SRC_DIR/libstd/prelude/v1.rs:LL:COL
-	   |
-	LL | pub use crate::option::Option::{self, Some, None};
-	   |                                             ---- the unit variant `None` is defined here
11	
12	error[E0530]: match bindings cannot shadow constants
13	  --> $DIR/issue-27033.rs:7:9
