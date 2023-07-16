
rustc 1.19.0-nightly (f1140a331 2017-05-08)
error: no associated item named `bat` found for type `Foo` in the current scope
 --> <anon>:9:5
  |
9 |     Foo::bat(());
  |     ^^^^^^^^
  |
  = note: the method `bat` exists but the following trait bounds were not satisfied: `Foo : Bar`, `Foo : Baz`, `&Foo : Bar`, `&Foo : Baz`, `&mut Foo : Bar`, `&mut Foo : Baz`
  = help: items from traits can only be used if the trait is implemented and in scope; the following trait defines an item `bat`, perhaps you need to implement it:
  = help: candidate #1: `Bat`
