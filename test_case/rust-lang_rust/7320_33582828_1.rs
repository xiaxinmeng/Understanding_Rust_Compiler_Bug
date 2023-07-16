
test.rs:2:25: 2:32 error: cannot pack type `~Self`, which does not fulfill `Send`, as a trait bounded by Send
test.rs:2     fn foo(~self) { bar(self as ~Foo); }
                                  ^~~~~~~
error: aborting due to previous error
