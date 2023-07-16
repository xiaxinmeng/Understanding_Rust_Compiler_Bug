
break.rs:3:1: 3:11 error: duplicate definition of type or module `Foo`
break.rs:3 mod Foo {}
           ^~~~~~~~~~
break.rs:2:1: 2:12 note: first definition of type or module `Foo` here
break.rs:2 impl Foo {}
           ^~~~~~~~~~~
error: aborting due to previous error
