
failures:
---- [ui] ui\invalid-module-declaration\invalid-module-declaration.rs stdout ----
	normalized stderr:
error[E0583]: file not found for module `baz`
  --> $DIR/auxiliary/foo/bar.rs:11:9
   |
11 | pub mod baz;
   |         ^^^
   |
   = help: name the file either bar/baz.rs or bar/baz/mod.rs inside the directory "C:/projects/rust/src/test/ui/invalid-module-declaration/auxiliary/foo"
error: aborting due to previous error
expected stderr:
error[E0583]: file not found for module `baz`
  --> $DIR/auxiliary/foo/bar.rs:11:9
   |
11 | pub mod baz;
   |         ^^^
   |
   = help: name the file either bar/baz.rs or bar/baz/mod.rs inside the directory "$DIR/auxiliary/foo"
error: aborting due to previous error
diff of stderr:
 error[E0583]: file not found for module `baz`
   --> $DIR/auxiliary/foo/bar.rs:11:9
    |
 11 | pub mod baz;
    |         ^^^
    |
-   = help: name the file either bar/baz.rs or bar/baz/mod.rs inside the directory "$DIR/auxiliary/foo"
+   = help: name the file either bar/baz.rs or bar/baz/mod.rs inside the directory "C:/projects/rust/src/test/ui/invalid-module-declaration/auxiliary/foo"
 
 error: aborting due to previous error
 