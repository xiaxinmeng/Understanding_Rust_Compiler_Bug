
---- [ui] ui\issue-10755.rs stdout ----
diff of stderr:
1	error: linker `llllll` not found
2	   |
-	   = note: No such file or directory (os error 2)
+	   = note: The system cannot find the file specified. (os error 2)
4	
5	error: aborting due to previous error
6	

--- [ui] ui\issue-23595-1.rs stdout ----
diff of stderr:
-	error[E0191]: the value of the associated type `ChildKey` (from the trait `Hierarchy`) must be specified
+	error[E0191]: the value of the associated type `Value` (from the trait `Hierarchy`) must be specified
2	  --> $DIR/issue-23595-1.rs:18:50
3	   |
4	LL |     type Children = Index<Self::ChildKey, Output=Hierarchy>;
-	   |                                                  ^^^^^^^^^ missing associated type `ChildKey` value
+	   |                                                  ^^^^^^^^^ missing associated type `Value` value
6	
7	error[E0191]: the value of the associated type `Children` (from the trait `Hierarchy`) must be specified
8	  --> $DIR/issue-23595-1.rs:18:50
10	LL |     type Children = Index<Self::ChildKey, Output=Hierarchy>;
11	   |                                                  ^^^^^^^^^ missing associated type `Children` value
12	
-	error[E0191]: the value of the associated type `Value` (from the trait `Hierarchy`) must be specified
+	error[E0191]: the value of the associated type `ChildKey` (from the trait `Hierarchy`) must be specified
14	  --> $DIR/issue-23595-1.rs:18:50
15	   |
16	LL |     type Children = Index<Self::ChildKey, Output=Hierarchy>;
-	   |                                                  ^^^^^^^^^ missing associated type `Value` value
+	   |                                                  ^^^^^^^^^ missing associated type `ChildKey` value
18	
19	error: aborting due to 3 previous errors
20	

---- [ui] ui\issue-43733-2.rs stdout ----
diff of stderr:
4	LL | static __KEY: Key<()> = Key::new();
5	   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::UnsafeCell<std::option::Option<()>>` cannot be shared between threads safely
6	   |
-	   = help: within `std::thread::Key<()>`, the trait `std::marker::Sync` is not implemented for `std::cell::UnsafeCell<std::option::Option<()>>`
-	   = note: required because it appears within the type `std::thread::Key<()>`
+	   = help: within `Key<()>`, the trait `std::marker::Sync` is not implemented for `std::cell::UnsafeCell<std::option::Option<()>>`
+	   = note: required because it appears within the type `Key<()>`
9	   = note: shared static variables must have a type that implements `Sync`
10	
11	error[E0277]: `std::cell::Cell<bool>` cannot be shared between threads safely
14	LL | static __KEY: Key<()> = Key::new();
15	   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::Cell<bool>` cannot be shared between threads safely
16	   |
-	   = help: within `std::thread::Key<()>`, the trait `std::marker::Sync` is not implemented for `std::cell::Cell<bool>`
-	   = note: required because it appears within the type `std::thread::Key<()>`
+	   = help: within `Key<()>`, the trait `std::marker::Sync` is not implemented for `std::cell::Cell<bool>`
+	   = note: required because it appears within the type `Key<()>`
19	   = note: shared static variables must have a type that implements `Sync`
20	
21	error: aborting due to 2 previous errors
