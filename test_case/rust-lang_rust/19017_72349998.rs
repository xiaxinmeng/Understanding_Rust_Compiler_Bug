
<anon>:4:26: 4:33 error: type `collections::vec::Vec<main::Test>` does not implement any method in scope named `clone`
<anon>:4     let w: Vec<Test> = v.clone();
                                  ^~~~~~~
<anon>:4:33: 4:33 help: methods from traits can only be called if the trait is implemented and in scope; the following trait defines a method `clone`, perhaps you need to implement it:
<anon>:4:33: 4:33 help: candidate #1: `core::clone::Clone`
error: aborting due to previous error
playpen: application terminated with error code 101
