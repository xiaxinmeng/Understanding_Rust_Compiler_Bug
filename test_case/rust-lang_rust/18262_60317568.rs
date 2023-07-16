
<anon>:53:21: 53:27 error: cannot borrow `*self.c` as immutable because it is also borrowed as mutable
<anon>:53         let inner = self.c.get();
                              ^~~~~~
<anon>:51:30: 51:36 note: previous borrow of `*self.c` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `*self.c` until the borrow ends
<anon>:51         let meh = create_foo(self.c);
                                       ^~~~~~
<anon>:56:6: 56:6 note: previous borrow ends here
<anon>:50     fn x(&'a mut self) {
...
<anon>:56     }
              ^
error: aborting due to previous error
