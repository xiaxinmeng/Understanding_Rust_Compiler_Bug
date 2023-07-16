
   |
16 |     let bar = foo.bar();
   |               ^^^------
   |               |
   |               borrowed value does not live long enough
   |               argument requires that `foo` is borrowed for `'static`
...
20 | }
   | - `foo` dropped here while still borrowed
