
Aug 19 09:54:47.599 INFO testing ldap3-0.5.0 against try#b96d5eea76e758bdd82fd4169eb4d5f7e3ce4fe2 for pr-43880
Aug 19 09:54:47.599 INFO running: cargo +b96d5eea76e758bdd82fd4169eb4d5f7e3ce4fe2 build --frozen
Aug 19 09:54:47.599 INFO creating container for: cargo +b96d5eea76e758bdd82fd4169eb4d5f7e3ce4fe2 build --frozen
Aug 19 09:54:47.599 INFO running `"docker" "create" "-v" "/home/ec2-user/cargobomb/./work/local/test-source/pr-43880/try#b96d5eea76e758bdd82fd4169eb4d5f7e3ce4fe2:/source:ro,Z" "-v" "/home/ec2-user/cargobomb/./work/local/target-dirs/pr-43880/try#b96d5eea76e758bdd82fd4169eb4d5f7e3ce4fe2:/target:rw,Z" "-v" "/home/ec2-user/cargobomb/./work/local/cargo-home:/cargo-home:ro,Z" "-v" "/home/ec2-user/cargobomb/./work/local/rustup-home:/rustup-home:ro,Z" "-e" "USER_ID=500" "-e" "CMD=cargo +b96d5eea76e758bdd82fd4169eb4d5f7e3ce4fe2 build --frozen" "cargobomb"`
Aug 19 09:54:47.673 INFO blam! 9fc7035ce68cfa6644c99cbc35ad850fd22cb7666409104c55eb40cfa186023c
Aug 19 09:54:47.674 INFO running `"docker" "start" "-a" "9fc7035ce68cfa6644c99cbc35ad850fd22cb7666409104c55eb40cfa186023c"`
Aug 19 09:54:48.186 INFO kablam!    Compiling ldap3 v0.5.0 (file:///source)
Aug 19 09:54:49.482 INFO kablam! error[E0308]: mismatched types
Aug 19 09:54:49.482 INFO kablam!    --> src/ldap.rs:235:42
Aug 19 09:54:49.482 INFO kablam!     |
Aug 19 09:54:49.482 INFO kablam! 235 |             let result = self.inner.call((req, Box::new(move |msgid| *closure_assigned_msgid.borrow_mut() = msgid))).select2(timeout).then(move |res| {
Aug 19 09:54:49.482 INFO kablam!     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait std::ops::Fn, found closure
Aug 19 09:54:49.482 INFO kablam!     |
Aug 19 09:54:49.482 INFO kablam!     = note: expected type `(ldap::LdapOp, std::boxed::Box<std::ops::Fn(i32) + 'static>)`
Aug 19 09:54:49.483 INFO kablam!                found type `(ldap::LdapOp, std::boxed::Box<[closure@src/ldap.rs:235:57: 235:114 closure_assigned_msgid:_]>)`
Aug 19 09:54:49.483 INFO kablam! 
Aug 19 09:54:49.489 INFO kablam! error[E0308]: mismatched types
Aug 19 09:54:49.489 INFO kablam!    --> src/ldap.rs:259:38
Aug 19 09:54:49.489 INFO kablam!     |
Aug 19 09:54:49.489 INFO kablam! 259 |             Box::new(self.inner.call((req, Box::new(|_msgid| ()))).and_then(|(tag, vec)| Ok(LdapResponse(tag, vec))))
Aug 19 09:54:49.489 INFO kablam!     |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait std::ops::Fn, found closure
Aug 19 09:54:49.489 INFO kablam!     |
Aug 19 09:54:49.489 INFO kablam!     = note: expected type `(ldap::LdapOp, std::boxed::Box<std::ops::Fn(i32) + 'static>)`
Aug 19 09:54:49.489 INFO kablam!                found type `(ldap::LdapOp, std::boxed::Box<[closure@src/ldap.rs:259:53: 259:64]>)`
Aug 19 09:54:49.489 INFO kablam! 
Aug 19 09:54:49.770 INFO kablam! error: aborting due to 2 previous errors
Aug 19 09:54:49.770 INFO kablam! 
Aug 19 09:54:49.793 INFO kablam! error: Could not compile `ldap3`.
Aug 19 09:54:49.793 INFO kablam! 
Aug 19 09:54:49.793 INFO kablam! To learn more, run the command again with --verbose.
Aug 19 09:54:49.796 INFO kablam! su: No module specific data is present
Aug 19 09:54:50.119 INFO running `"docker" "rm" "-f" "9fc7035ce68cfa6644c99cbc35ad850fd22cb7666409104c55eb40cfa186023c"`
Aug 19 09:54:50.136 INFO blam! 9fc7035ce68cfa6644c99cbc35ad850fd22cb7666409104c55eb40cfa186023c
