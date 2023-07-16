console
warning: unused boxed `Fn` trait object that must be used
  --> ./p/unused.rs:11:5
   |
11 |     xs.remove(1);
   |     ^^^^^^^^^^^^
   |
   = note: closures are lazy and do nothing unless called
   = note: `#[warn(unused_must_use)]` on by default
