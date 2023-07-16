
$ export RUST_LOG=all
$ ./debug
rust: ~"\"error\""                                                                                                                                                   
$ export RUST_LOG=error
$ ./debug
rust: ~"\"error\""
$ export RUST_LOG=warning
$ ./debug
rust: ~"\"error\""
$ export RUST_LOG=info
$ ./debug
rust: ~"\"error\""
$ export RUST_LOG=debug
$ ./debug
rust: ~"\"error\""
rust: ~"\"warn\""
rust: ~"\"info\""
rust: ~"\"debug\""
