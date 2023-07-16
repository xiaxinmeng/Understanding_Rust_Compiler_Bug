
foo.rs:4:9: 4:12 error: import `Foo` conflicts with type in this module
foo.rs:4     use Foo;
                 ^~~
foo.rs:6:5: 8:6 note: note conflicting type here
foo.rs:6     impl Foo {
foo.rs:7         fn baz(&self) {}
foo.rs:8     }
error: aborting due to previous error
