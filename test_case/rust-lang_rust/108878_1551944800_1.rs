console
>  strace ./target/debug/reproducer 
execve("./target/debug/reproducer", ["./target/debug/reproducer"], 0x7ffcf1583680 /* 77 vars */) = 0
arch_prctl(ARCH_SET_FS, 0x7f42a765c8e8) = 0
set_tid_address(0x7f42a765ca98)         = 13314
poll([{fd=0, events=0}, {fd=1, events=0}, {fd=2, events=0}], 3, 0) = 0 (Timeout)
rt_sigaction(SIGPIPE, {sa_handler=SIG_IGN, sa_mask=[], sa_flags=SA_RESTORER|SA_RESTART, sa_restorer=0x7f42a7636006}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGSEGV, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigprocmask(SIG_UNBLOCK, [RT_1 RT_2], NULL, 8) = 0
rt_sigaction(SIGSEGV, {sa_handler=0x7f42a760d1a0, sa_mask=[], sa_flags=SA_RESTORER|SA_ONSTACK|SA_SIGINFO, sa_restorer=0x7f42a7636006}, NULL, 8) = 0
rt_sigaction(SIGBUS, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGBUS, {sa_handler=0x7f42a760d1a0, sa_mask=[], sa_flags=SA_RESTORER|SA_ONSTACK|SA_SIGINFO, sa_restorer=0x7f42a7636006}, NULL, 8) = 0
sigaltstack(NULL, {ss_sp=NULL, ss_flags=SS_DISABLE, ss_size=0}) = 0
mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0x7f42a75e3000
mprotect(0x7f42a75e3000, 4096, PROT_NONE) = 0
sigaltstack({ss_sp=0x7f42a75e4000, ss_flags=0, ss_size=8192}, NULL) = 0
brk(NULL)                               = 0x555556e6f000
brk(0x555556e71000)                     = 0x555556e71000
mmap(0x555556e6f000, 4096, PROT_NONE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x555556e6f000
mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f42a75e2000
rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [], 8) = 0
rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
--- SIGSEGV {si_signo=SIGSEGV, si_code=SEGV_MAPERR, si_addr=NULL} ---
rt_sigaction(SIGSEGV, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=SA_RESTORER, sa_restorer=0x7f42a7636006}, NULL, 8) = 0
rt_sigreturn({mask=[]})                 = 139924253016624
--- SIGSEGV {si_signo=SIGSEGV, si_code=SEGV_MAPERR, si_addr=NULL} ---
+++ killed by SIGSEGV +++
zsh: segmentation fault  strace ./target/debug/reproducer
