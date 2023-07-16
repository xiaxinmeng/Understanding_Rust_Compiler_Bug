
test.rs:5:5: 5:15 error: duplicate definition of value `B`
test.rs:5     fn B() { }
              ^~~~~~~~~~
test.rs:4:5: 4:23 note: first definition of value `B` here
test.rs:4     pub struct B(int);
              ^~~~~~~~~~~~~~~~~~
