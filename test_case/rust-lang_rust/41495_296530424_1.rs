
rustc 1.16.0 (30cf806ef 2017-03-10)
error: `container` does not live long enough
  --> <anon>:12:1
   |
11 |     if let true = container.try_borrow().is_ok() { }
   |                   --------- borrow occurs here
12 | }
   | ^ `container` dropped here while still borrowed
   |
   = note: values in a scope are dropped in the opposite order they are created

error: `container` does not live long enough
  --> <anon>:20:1
   |
16 |     match container.try_borrow().is_ok() {
   |           --------- borrow occurs here
...
20 | }
   | ^ `container` dropped here while still borrowed
   |
   = note: values in a scope are dropped in the opposite order they are created

error: aborting due to 2 previous errors
