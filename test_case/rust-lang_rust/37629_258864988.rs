
error: trait `RLINQ` is private
  --> <anon>:15:9
   |
15 |     use rlinq::RLINQ;
   |         ^^^^^^^^^^^^

error: no method named `is_where` found for type `std::vec::Vec<std::string::String>` in the current scope
  --> <anon>:19:11
   |
19 |         x.is_where();
   |           ^^^^^^^^
   |
   = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
   = help: candidate #1: `use rlinq::RLINQ`

error: aborting due to previous error
