rust
error: no method named `f` found for type `{integer}` in the current scope
  --> $DIR/issue_41652.rs:19:11
   |
19 |         3.f()
   |           ^
   |
   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: candidate #1 is defined in the trait `issue_41652_b::Tr`
   = help: to disambiguate the method call, write `issue_41652_b::Tr::f(3)` instead

error: aborting due to previous error
