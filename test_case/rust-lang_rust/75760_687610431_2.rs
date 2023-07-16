diff
---- [ui] ui/traits/cycle-cache-err-60010.rs stdout ----
diff of stderr:

6	   |
7	   = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
8	
-	error[E0275]: overflow evaluating the requirement `Runtime<RootDatabase>: RefUnwindSafe`
-	  --> $DIR/cycle-cache-err-60010.rs:31:20
-	   |
-	LL | trait Database {
-	   |       -------- required by a bound in this
-	LL |     type Storage;
-	   |     ------------- required by this bound in `Database`
-	...
-	LL |     type Storage = SalsaStorage;
-	   |                    ^^^^^^^^^^^^
-	   |
-	   = note: required because it appears within the type `RootDatabase`
-	   = note: required because of the requirements on the impl of `SourceDatabase` for `RootDatabase`
-	   = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
-	   = note: required because it appears within the type `SalsaStorage`
-	
-	error: aborting due to 2 previous errors
+	error: aborting due to previous error
26	
27	For more information about this error, try `rustc --explain E0275`.
28	
