
% ./x86_64-apple-darwin/stage2/bin/rustc -v
./x86_64-apple-darwin/stage2/bin/rustc 0.8-pre (9c22f65 2013-07-15 05:01:19 -0700)
host: x86_64-apple-darwin
% ./x86_64-apple-darwin/stage2/bin/rustc /tmp/foo.rs
/tmp/foo.rs:4:13: 4:18 error: can't capture dynamic environment in a fn item; use the || { ... } closure form instead
/tmp/foo.rs:4     fn x() { self.m() }
                           ^~~~~
/tmp/foo.rs:4:13: 4:18 error: internal compiler error: self wasn't mapped to a def?!
/tmp/foo.rs:4     fn x() { self.m() }
                           ^~~~~
% 
