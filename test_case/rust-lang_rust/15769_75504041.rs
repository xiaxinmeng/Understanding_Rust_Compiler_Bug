
test.rs:1:5: 1:9 error: import `test` conflicts with existing submodule [E0258]
test.rs:1 use test;
              ^~~~
test.rs:2:1: 2:12 note: note conflicting module here
test.rs:2 mod test {}
          ^~~~~~~~~~~
error: aborting due to previous error
