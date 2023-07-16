 rust
   #[pkg("Foo")]
   extern mod foo; // Link crate Foo from package Foo, and rename it to foo
   #[pkg("Foo"), crate("Bar")]
   extern mod foo; // Link crate Bar from package Foo, and rename it to foo
   