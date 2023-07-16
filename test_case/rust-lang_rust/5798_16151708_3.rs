
loan.rs:11:28: 11:32 error: loan of dereference of mutable & pointer as immutable conflicts with prior loan
loan.rs:11         self.takes_mut_self(self.takes_self());
                                        ^~~~
loan.rs:11:8: 11:12 note: prior loan as mutable granted here
loan.rs:11         self.takes_mut_self(self.takes_self());
                    ^~~~
error: aborting due to previous error
