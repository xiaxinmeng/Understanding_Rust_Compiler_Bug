
foo.rs:2:28: 2:31 error: constants cannot refer to other statics, insert an intermediate constant instead [E0013]
foo.rs:2 const BAR: &'static u32 = &FOO;
                                    ^~~
error: aborting due to previous error
