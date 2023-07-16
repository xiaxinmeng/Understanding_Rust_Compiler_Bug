
/Users/tchevalier/rust/src/libcore/send_map.rs:227:22: 227:38 error: illegal borrow unless pure: unique value in aliasable, mutable location
/Users/tchevalier/rust/src/libcore/send_map.rs:227             swap(&mut self.buckets[idx], &mut x);
                                                                         ^~~~~~~~~~~~~~~~
/Users/tchevalier/rust/src/libcore/send_map.rs:227:12: 227:16 note: impure due to access to impure function
/Users/tchevalier/rust/src/libcore/send_map.rs:227             swap(&mut self.buckets[idx], &mut x);
