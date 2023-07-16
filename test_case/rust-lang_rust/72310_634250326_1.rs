
error[E0637]: `&` without an explicit lifetime name cannot be used here
    --> src/libcore/iter/adapters/mod.rs:1635:11
     |
1635 |           &I::Item: PartialEq<&R>,
     |           ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
    --> src/libcore/iter/adapters/mod.rs:1635:31
     |
1635 |           &I::Item: PartialEq<&R>,
     |                               ^ explicit lifetime name needed here
