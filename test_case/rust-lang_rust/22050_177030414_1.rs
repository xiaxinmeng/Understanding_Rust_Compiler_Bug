
<anon>:3:20: 3:39 error: no method named `write_all` found for type `&mut [u8]` in the current scope
<anon>:3     (&mut buf[..]).write_all(&b"test");
                            ^~~~~~~~~~~~~~~~~~~
<anon>:3:20: 3:39 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
<anon>:3:20: 3:39 help: candidate #1: use `std::io::Write`
