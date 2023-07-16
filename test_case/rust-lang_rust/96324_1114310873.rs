plain
.............................ii......................................................... 440/1200
........................i....i.......................................................... 528/1200
......i......................ii......................................................... 616/1200
........................................................................................ 704/1200
.............................................................F....F..................... 792/1200
................................................................................iii..... 968/1200
........................................................................................ 1056/1200
...........................................................iiiiii....................... 1144/1200
........................i...............................
........................i...............................
failures:

---- src/os/linux/net.rs - os::linux::net::TcpStreamExt::quickack (line 43) stdout ----
error[E0599]: no method named `set_quickack` found for struct `TcpStream` in the current scope
  --> src/os/linux/net.rs:49:8
   |
9  | stream.set_quickack(true).expect("set_quickack call failed");
   |        ^^^^^^^^^^^^ method not found in `TcpStream`
  ::: /checkout/library/std/src/os/linux/net.rs:35:8
   |
   |
35 |     fn set_quickack(&self, quickack: bool) -> io::Result<()>;
   |        ------------ the method is available for `TcpStream` here
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
4  | use std::os::linux::net::TcpStreamExt;
4  | use std::os::linux::net::TcpStreamExt;
   |

error[E0599]: no method named `quickack` found for struct `TcpStream` in the current scope
  --> src/os/linux/net.rs:50:19
   |
10 | assert_eq!(stream.quickack().unwrap_or(false), true);
   |                   ^^^^^^^^ method not found in `TcpStream`
  ::: /checkout/library/std/src/os/linux/net.rs:53:8
   |
   |
53 |     fn quickack(&self) -> io::Result<bool>;
   |        -------- the method is available for `TcpStream` here
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
4  | use std::os::linux::net::TcpStreamExt;
4  | use std::os::linux::net::TcpStreamExt;
   |

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---- src/os/linux/net.rs - os::linux::net::TcpStreamExt::set_quickack (line 26) stdout ----
error[E0599]: no method named `set_quickack` found for struct `TcpStream` in the current scope
  --> src/os/linux/net.rs:32:8
   |
9  | stream.set_quickack(true).expect("set_quickack call failed");
   |        ^^^^^^^^^^^^ method not found in `TcpStream`
  ::: /checkout/library/std/src/os/linux/net.rs:35:8
   |
   |
35 |     fn set_quickack(&self, quickack: bool) -> io::Result<()>;
   |        ------------ the method is available for `TcpStream` here
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
4  | use std::os::linux::net::TcpStreamExt;
---
For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.

failures:
    src/os/linux/net.rs - os::linux::net::TcpStreamExt::quickack (line 43)
    src/os/linux/net.rs - os::linux::net::TcpStreamExt::set_quickack (line 26)
test result: FAILED. 1180 passed; 2 failed; 18 ignored; 0 measured; 0 filtered out; finished in 18.74s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:22:35
