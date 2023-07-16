plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0308]: mismatched types
    --> library/std/src/io/util/tests.rs:133:30
     |
133  |     assert_eq!(Write::by_ref(n).write(&[0; 1024]).unwrap(), 1024);
     |                |             |
     |                |             |
     |                |             expected `&mut _`, found struct `util::Null`
     |                |             help: consider mutably borrowing here: `&mut n`
     |
     = note: expected mutable reference `&mut _`
                           found struct `util::Null`
note: associated function defined here
---

error[E0308]: mismatched types
   --> library/std/src/io/util/tests.rs:142:29
    |
142 |     assert_eq!(Read::by_ref(n).read(&mut [0; 1024]).unwrap(), 0);
    |                |            |
    |                |            |
    |                |            expected `&mut _`, found struct `util::Null`
    |                |            help: consider mutably borrowing here: `&mut n`
    |
    = note: expected mutable reference `&mut _`
                          found struct `util::Null`
note: associated function defined here
---

error[E0308]: mismatched types
   --> library/std/src/io/util/tests.rs:164:18
    |
164 |     Read::by_ref(n).read_buf(&mut buf).unwrap();
    |     |            |
    |     |            |
    |     |            expected `&mut _`, found struct `util::Null`
    |     |            help: consider mutably borrowing here: `&mut n`
    |
    = note: expected mutable reference `&mut _`
                          found struct `util::Null`
note: associated function defined here
