 rust
   extern mod foo = "Foo"; // Link crate Foo from package Foo, and rename it to foo
   
   #[crate(Bar)]
   extern mod foo = "Foo"; // Link crate Boo from package Foo, and rename it to foo
   