
$ strace -e trace=write cargo -q run --release | grep -q Hello; echo "Exit status" $?
write(6, "|", 1)                        = 1
write(6, "|", 1)                        = 1
write(6, "|", 1)                        = 1
write(6, "|", 1)                        = 1
write(6, "|", 1)                        = 1
write(6, "|", 1)                        = 1
write(6, "|", 1)                        = 1
write(6, "|", 1)                        = 1
write(1, "Hello, World!\n", 14)         = 14
write(1, "Hello, World!\n", 14)         = 14
write(1, "Hello, World!\n", 14)         = 14
write(1, "Hello, World!\n", 14)         = 14
write(1, "Hello, World!\n", 14)         = 14
write(1, "Hello, World!\n", 14)         = -1 EPIPE (Broken pipe)
--- SIGPIPE {si_signo=SIGPIPE, si_code=SI_USER, si_pid=28605, si_uid=1000} ---
write(1, "Hello, World!\n", 14)         = -1 EPIPE (Broken pipe)
--- SIGPIPE {si_signo=SIGPIPE, si_code=SI_USER, si_pid=28605, si_uid=1000} ---
write(2, "thread '", 8thread ')                 = 8
write(2, "main", 4main)                     = 4
write(2, "' panicked at '", 15' panicked at ')         = 15
write(2, "failed printing to stdout: Broke"..., 52failed printing to stdout: Broken pipe (os error 32)) = 52
write(2, "', ", 3', )                      = 3
write(2, "libstd/io/stdio.rs", 18libstd/io/stdio.rs)      = 18
write(2, ":", 1:)                        = 1
write(2, "700", 3700)                      = 3
write(2, ":", 1:)                        = 1
write(2, "9", 19)                        = 1
write(2, "\n", 1
)                       = 1
write(2, "note: Run with `RUST_BACKTRACE=1"..., 51note: Run with `RUST_BACKTRACE=1` for a backtrace.
) = 51
write(1, "Hello, World!\n", 14)         = -1 EPIPE (Broken pipe)
--- SIGPIPE {si_signo=SIGPIPE, si_code=SI_USER, si_pid=28605, si_uid=1000} ---
+++ exited with 101 +++
Exit status 0
