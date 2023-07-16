
error[E0599]: no method named `aay` found for struct `A` in the current scope
  --> src/main.rs:16:7
   |
10 | struct A;
   | --------- method `aay` not found for this
...
16 |     A.aay();
   |       ^^^ help: there is an associated function with a similar name: `away`
