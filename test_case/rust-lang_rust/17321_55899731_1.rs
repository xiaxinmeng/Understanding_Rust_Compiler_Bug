
foo.rs:6:9: 6:15 error: import `Foo` conflicts with type in this module
foo.rs:6     use a::Foo;
                 ^~~~~~
foo.rs:7:5: 9:6 note: note conflicting type here
foo.rs:7     impl Foo { // ERROR: found value name used as type
foo.rs:8         fn bar(&self) { }
foo.rs:9     }
error: aborting due to previous error
