
mut-self-issues.rs:12:28: 12:32 error: loan of dereference of mutable & pointer as immutable conflicts with prior loan
mut-self-issues.rs:12         self.takes_mut_self(self.takes_self());
                                                  ^~~~
mut-self-issues.rs:12:8: 12:12 note: prior loan as mutable granted here
mut-self-issues.rs:12         self.takes_mut_self(self.takes_self());
                              ^~~~
error: aborting due to previous error
