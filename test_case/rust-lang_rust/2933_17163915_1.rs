
% rustc /tmp/bug5.rs
/tmp/bug5.rs:6:12: 6:23 error: multiple applicable methods in scope
/tmp/bug5.rs:6     assert!(22.foo() == ~"int");
                           ^~~~~~~~~~~
<core-macros>:52:4: 69:5 note: in expansion of assert!
/tmp/bug5.rs:6:4: 6:32 note: expansion site
/tmp/bug5.rs:2:24: 2:57 note: candidate #1 is `__extensions__::foo`
/tmp/bug5.rs:2 impl methods for uint { fn foo(&self) -> ~str { ~"uint" } }
                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/tmp/bug5.rs:3:24: 3:57 note: candidate #2 is `__extensions__::foo`
/tmp/bug5.rs:3 impl methods for int  { fn foo(&self) -> ~str { ~"int"  } }
                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
