
error[E0599]: no method named `foo` found for reference `&B` in the current scope
  --> <source>:17:15
   |
17 |             b.foo(); //~ ERROR: no method named `foo` found
   |               ^^^ method not found in `&B`
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
14 |         use a::A;
   |
