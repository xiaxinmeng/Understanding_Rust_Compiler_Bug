
> #0  0x00007f7fe3a0c3aa in ___lwp_park60 () from /usr/libexec/ld.elf_so
> #1  0x00007f7fe3a01595 in _rtld_shared_enter () from /usr/libexec/ld.elf_so
> #2  0x00007f7fe3a00b9b in _rtld_bind () from /usr/libexec/ld.elf_so
> #3  0x00007f7fe3a007fd in _rtld_bind_start () from /usr/libexec/ld.elf_so
> #4  0x0000000000000246 in ?? ()
> #5  0x000075686ce431fa in _lwp_ctl () from /usr/lib/libc.so.12
> #6  0x000075686cf06cf9 in je_jemalloc_prefork () from /usr/lib/libc.so.12
> #7  0x000075686d6f3000 in ?? ()
> #8  0x0000000000000001 in ?? ()
> #9  0x0000000000400b89 in spawn () at t.c:20
> Backtrace stopped: previous frame inner to this frame (corrupt stack?)
> 