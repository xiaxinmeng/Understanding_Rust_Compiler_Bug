
6:9: 6:20 error: cannot assign to `**borrow` because it is borrowed [E0506]
6         *borrow = 1;
          ^~~~~~~~~~~
5:41: 5:47 note: borrow of `**borrow` occurs here
5         let borrow_borrow: &&mut i32 = &borrow;
                                          ^~~~~~
