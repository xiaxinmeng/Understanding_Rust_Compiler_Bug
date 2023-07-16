
error: dangling pointer was dereferenced
    --> /checkout/src/libcollections/vec.rs:1127:9
     |
1127 | 
     |         ^^^^^^^^
     |
note: inside call to <std::vec::Vec<T>><Item>::len
    --> foobar.rs:14:18
     |
14   |         unsafe { (*self.list).values.len() }
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside call to Item::get_len
    --> foobar.rs:47:16
     |
47   |     assert_eq!(list.values[0].get_len(), 1);
     |                ^^^^^^^^^^^^^^^^^^^^^^^^
note: inside call to main
