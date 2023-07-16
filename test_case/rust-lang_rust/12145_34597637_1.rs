
foo8.rs:6:25: 6:35 error: multiple applicable methods in scope
foo8.rs:6         fn say(&self) { self.say() }
                                  ^~~~~~~~~~
foo8.rs:6:9: 6:37 note: candidate #1 is `Even$$BP$E::say`
foo8.rs:6         fn say(&self) { self.say() }
                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
foo8.rs:14:9: 14:37 note: candidate #2 is `Odd$$BP$O::say`
foo8.rs:14         fn say(&self) { self.say() }
                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
foo8.rs:14:25: 14:35 error: multiple applicable methods in scope
foo8.rs:14         fn say(&self) { self.say() }
                                   ^~~~~~~~~~
foo8.rs:14:9: 14:37 note: candidate #1 is `Odd$$BP$O::say`
foo8.rs:14         fn say(&self) { self.say() }
                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
foo8.rs:6:9: 6:37 note: candidate #2 is `Even$$BP$E::say`
foo8.rs:6         fn say(&self) { self.say() }
                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
