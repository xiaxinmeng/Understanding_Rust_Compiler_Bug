
hello.rs:10:1: 10:10 error: inherent implementations are only allowed on types defined in the current module [E0257]
hello.rs:10 impl T {} // oops
            ^~~~~~~~~
hello.rs:1:5: 1:9 note: import from other module here
hello.rs:1 use t::T;
               ^~~~

