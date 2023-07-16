
mut-self-issues.rs:12:28: 12:33 error: cannot borrow `*self` as immutable because it is also borrowed as mutable
mut-self-issues.rs:12         self.takes_mut_self(self.takes_self());
                                                  ^~~~~
mut-self-issues.rs:12:8: 12:13 note: second borrow of `*self` occurs here
mut-self-issues.rs:12         self.takes_mut_self(self.takes_self());
                              ^~~~~
error: aborting due to previous error
