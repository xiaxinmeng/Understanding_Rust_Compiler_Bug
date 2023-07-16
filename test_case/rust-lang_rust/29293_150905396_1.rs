
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
 77.45    0.026666        8889         3           futex
 19.98    0.006879          12       558           read
  1.38    0.000475           7        66           mprotect
  0.58    0.000201           4        47        12 open
  0.38    0.000131           1        99           mmap
  0.23    0.000078           2        33           fstat
  0.00    0.000000           0         8           write
  0.00    0.000000           0        35           close
  0.00    0.000000           0        10         8 stat
  0.00    0.000000           0        24           lseek
  0.00    0.000000           0        12           munmap
  0.00    0.000000           0         4           brk
  0.00    0.000000           0         5           rt_sigaction
  0.00    0.000000           0         1           rt_sigprocmask
  0.00    0.000000           0         2           ioctl
  0.00    0.000000           0         1         1 access
  0.00    0.000000           0        23           madvise
  0.00    0.000000           0         1           clone
  0.00    0.000000           0         1           execve
  0.00    0.000000           0         2         1 readlink
  0.00    0.000000           0         2           getrlimit
  0.00    0.000000           0         2           sigaltstack
  0.00    0.000000           0         1           prctl
  0.00    0.000000           0         1           arch_prctl
  0.00    0.000000           0         2           sched_getaffinity
  0.00    0.000000           0         1           set_tid_address
  0.00    0.000000           0         2           set_robust_list
  0.00    0.000000           0         2           getrandom
------ ----------- ----------- --------- --------- ----------------
100.00    0.034430                   948        22 total
