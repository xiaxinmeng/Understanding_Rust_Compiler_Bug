
error[E0599]: no variant named `mispellable` found for type `Enum` in the current scope
  --> file2.rs:16:11
   |
1  | enum Enum { Variant }
   | --------- variant `mispellable` not found here
...
16 |     Enum::mispellable();
   |     ------^^^^^^^^^^^
   |     |
   |     variant not found in `Enum`
   |     help: did you mean: `misspellable`

error[E0599]: no variant named `mispellable_trait` found for type `Enum` in the current scope
  --> file2.rs:17:11
   |
1  | enum Enum { Variant }
   | --------- variant `mispellable_trait` not found here
...
17 |     Enum::mispellable_trait();
   |     ------^^^^^^^^^^^^^^^^^
   |     |
   |     variant not found in `Enum`
