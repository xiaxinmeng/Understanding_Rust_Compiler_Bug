
[15:57:32] <kmc>     yeah, you can install a handler for SIGSEGV
[15:57:39] <@pnkfelix>   tjc: sigaction I think
[15:57:48] <strcat>  pnkfelix: we can't use sigaction
[15:57:53] <strcat>  we mess up the stack and use threads
[15:57:57] <acrichto>    it's tricky to do correctly
[15:57:58] <kmc>     and yeah, the siginfo_t provides the address which faulted
