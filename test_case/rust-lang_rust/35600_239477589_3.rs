
error: no method named `bar` found for type `char` in the current scope
 --> <anon>:9:9
  |
9 |     'a'.bar();
  |         ^^^
  |
  = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: candidate #1 is defined in the trait `Foo`
 --> <anon>:2:5
  |
2 |     type bar;
  |     ^^^^^^^^^
  = help: items from traits can only be used if the trait is implemented and in scope; the following trait defines an item `bar`, perhaps you need to implement it:
  = help: candidate #1: `Foo`
