
[jdm@rosencrantz traitinheritance]$ rustc test3.rs 
test3.rs:33:4: 33:19 error: type `@B` does not implement any method in scope named `foo`
test3.rs:33     (5 as B).foo();
                ^~~~~~~~~~~~~~~
