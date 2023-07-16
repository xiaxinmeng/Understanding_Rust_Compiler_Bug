
wtf3.rs:7:22: 7:27 error: cannot borrow `*self` as mutable because it is also borrowed as immutable
wtf3.rs:7         self.foo.call(self);
                                ^~~~~
wtf3.rs:7:8: 7:16 note: second borrow of `*self` occurs here
wtf3.rs:7         self.foo.call(self);
                  ^~~~~~~~
