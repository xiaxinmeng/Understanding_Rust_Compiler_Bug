
clone3({flags=CLONE_VM|CLONE_VFORK, exit_signal=SIGCHLD, stack=0x7f11a1211000, stack_size=0x9000}, 88strace: Process 148962 attached
 <unfinished ...>
[pid 148962] rt_sigprocmask(SIG_BLOCK, NULL, ~[KILL STOP], 8) = 0
[...]
[pid 148962] rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
[pid 148962] execve("/no-binary-by-this-name-should-exist", ["/no-binary-by-this-name-should-e"...], 0x7ffe09be3e28 /* 69 vars */) = -1 ENOENT (No such file or directory)
[pid 148962] exit_group(127)            = ?
[pid 148961] <... clone3 resumed>)      = 148962
[pid 148962] +++ exited with 127 +++
wait4(148962, NULL, 0, NULL)            = 148962
