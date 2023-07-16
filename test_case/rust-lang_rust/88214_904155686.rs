
---- [ui (nll)] ui/borrowck/issue-82462.rs stdout ----
diff of stderr:

7	   |              |               immutable borrow occurs here
8	   |              a temporary with access to the immutable borrow is created here ...
9	LL |         v.push(*x);
-	   |         ^^^^^^^^^^ mutable borrow occurs here
+	   |         ^ mutable borrow occurs here
11	LL |         break;
12	LL |     }
13	   |     - ... and the immutable borrow might be used here, when that temporary is dropped and runs the `Drop` code for type `DroppingSlice`
