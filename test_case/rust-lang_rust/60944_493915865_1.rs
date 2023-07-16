text
error[E0106]: missing lifetime specifier
   |
LL |     fn b(self: &Box<Foo>, f: &Foo) -> &Foo { f }
   |                                       ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `self` or `f`
