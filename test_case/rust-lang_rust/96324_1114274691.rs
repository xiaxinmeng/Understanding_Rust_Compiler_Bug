plain

error: implementation has missing stability attribute
  --> library/std/src/os/linux/net.rs:57:1
   |
57 | impl Sealed for net::TcpStream {}

error: implementation has missing stability attribute
  --> library/std/src/os/linux/net.rs:59:1
   |
   |
59 | / impl TcpStreamExt for net::TcpStream { 
60 | |     fn set_quickack(&self, quickack: bool) -> io::Result<()> {
61 | |         self.as_inner().set_quickack(quickack)
...  |
66 | |     }
67 | | }
   | |_^
   | |_^

error[E0599]: no method named `as_inner` found for reference `&tcp::TcpStream` in the current scope
  --> library/std/src/os/linux/net.rs:61:14
   |
61 |         self.as_inner().set_quickack(quickack)
   |              ^^^^^^^^ method not found in `&tcp::TcpStream`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
7  | use crate::sys_common::AsInner;
7  | use crate::sys_common::AsInner;
   |

error[E0599]: no method named `as_inner` found for reference `&tcp::TcpStream` in the current scope
  --> library/std/src/os/linux/net.rs:65:14
   |
65 |         self.as_inner().quickack()
   |              ^^^^^^^^ method not found in `&tcp::TcpStream`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
7  | use crate::sys_common::AsInner;
