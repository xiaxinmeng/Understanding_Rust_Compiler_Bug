
hello.rs:1:1: 1:43 error: in type `&'a &'b &'c ()`, reference has a longer lifetime than the data it references
hello.rs:1 struct Foo<'a,'b,'c> { p: &'a &'b &'c () }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:1:1: 1:43 note: the pointer is valid for the lifetime 'a as defined on the struct at 1:0
hello.rs:1 struct Foo<'a,'b,'c> { p: &'a &'b &'c () }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:1:1: 1:43 note: but the referenced data is only valid for the lifetime 'b as defined on the struct at 1:0
hello.rs:1 struct Foo<'a,'b,'c> { p: &'a &'b &'c () }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:1:1: 1:43 error: in type `&'b &'c ()`, reference has a longer lifetime than the data it references
hello.rs:1 struct Foo<'a,'b,'c> { p: &'a &'b &'c () }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:1:1: 1:43 note: the pointer is valid for the lifetime 'b as defined on the struct at 1:0
hello.rs:1 struct Foo<'a,'b,'c> { p: &'a &'b &'c () }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:1:1: 1:43 note: but the referenced data is only valid for the lifetime 'c as defined on the struct at 1:0
hello.rs:1 struct Foo<'a,'b,'c> { p: &'a &'b &'c () }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors

