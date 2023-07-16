shell
$ clear; strace -e open,openat -ff cargo t test_string_highlighting
(lots of output omitted)
[pid 38722] openat(AT_FDCWD, "/home/amos/bearcove/rust/crates/ide/src/syntax_highlighting/./test_data/highlight_strings.html", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
