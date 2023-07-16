
error[E0373]: closure may outlive the current function, but it borrows `self`, which is owned by the current function
  --> src/lib.rs:13:49
   |
13 |         self.data[start..max].iter().take_while(|elem| (self.key_fn)(elem) == *key)
   |                                                 ^^^^^^ ------------- `self` is borrowed here
   |                                                 |
   |                                                 may outlive borrowed value `self`
   |
note: closure is returned here
  --> src/lib.rs:13:9
   |
13 |         self.data[start..max].iter().take_while(|elem| (self.key_fn)(elem) == *key)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
   |
13 |         self.data[start..max].iter().take_while(move |elem| (self.key_fn)(elem) == *key)
   |                                                 ++++

error[E0373]: closure may outlive the current function, but it borrows `key`, which is owned by the current function
  --> src/lib.rs:13:49
   |
13 |         self.data[start..max].iter().take_while(|elem| (self.key_fn)(elem) == *key)
   |                                                 ^^^^^^                        ---- `key` is borrowed here
   |                                                 |
   |                                                 may outlive borrowed value `key`
   |
note: closure is returned here
  --> src/lib.rs:13:9
   |
13 |         self.data[start..max].iter().take_while(|elem| (self.key_fn)(elem) == *key)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `key` (and any other referenced variables), use the `move` keyword
   |
13 |         self.data[start..max].iter().take_while(move |elem| (self.key_fn)(elem) == *key)
   |                                                 ++++
