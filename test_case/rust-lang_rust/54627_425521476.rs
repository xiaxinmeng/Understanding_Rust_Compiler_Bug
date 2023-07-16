
$ pwd
/Users/mehcode/Workspace/github.com/huonw/primal

$ lldb ~/.cargo/bin/cargo
(lldb) process launch -- +beta build --target armv7-apple-ios
Compiling primal-sieve v0.2.9 [...]
[... waited about 10 minutes ...]
[ hit ctrl-c ]

(lldb) bt all
