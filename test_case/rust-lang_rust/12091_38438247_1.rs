
test.rs:22:2: 22:12 error: multiple applicable methods in scope
test.rs:22      (S1).bar();
                ^~~~~~~~~~
test.rs:10:2: 10:15 note: candidate #1 is `Bar2::bar`
test.rs:10      fn bar(&self) {}
                ^~~~~~~~~~~~~
test.rs:5:2: 5:15 note: candidate #2 is `Bar1::bar`
test.rs:5       fn bar(&self) {}
                ^~~~~~~~~~~~~
test.rs:22:2: 22:12 error: failed to find an implementation of trait Foo for S1
test.rs:22      (S1).bar();
