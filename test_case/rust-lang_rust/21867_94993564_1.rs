 shell
$ rustc main.rs
main.rs:6:41: 6:60 error: ambiguous associated type; specify the type using the syntax `<Type as std::net::addr::ToSocketAddrs>::Iter` [E0223]
main.rs:6     fn to_socket_addrs(&self) -> Result<ToSocketAddrs::Iter> {
                                                  ^~~~~~~~~~~~~~~~~~~
main.rs:6:34: 6:61 error: wrong number of type arguments: expected 2, found 1 [E0243]
main.rs:6     fn to_socket_addrs(&self) -> Result<ToSocketAddrs::Iter> {
                                           ^~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
