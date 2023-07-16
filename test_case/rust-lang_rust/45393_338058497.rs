
new features:
- RTLD_LAZY deferred symbol binding, functionally equivalent to lazy binding
- safeguard against dlopen of multiple libc versions/instances
- new posix_spawn flag POSIX_SPAWN_SETSID
- posix_spawnattr_setflags now reports unknown flags as error
- ldso option --argv0 to set argv[0]
- added _NL_LOCALE_NAME extension to nl_langinfo

compatibility:
- dlopen local-to-global promotion no longer changes existing symbols
- gettext now searches locale name variants for translation files
- increased locale name length limit from 15 to 23 bytes
- setlocale(LC_ALL, 0) returns single name if all categories are same
- realloc no longer fails when mremap doesn't work
- getservby* no longer treat numeric port strings as service records
- mmap now works around incorrect EPERM error codes from kernel
- impact of REG_* namespace pollution in x86[_64] signal.h is reduced
- arm atomic asm now assembles correctly with new binutils
- PAGE_SIZE on arm is no longer constant (quiet upstream ABI relaxation)
- lsearch/lfind now pass args to compare callback in canonical order
- STB_WEAK and STB_GNU_UNIQUE symbols now behave same as STB_GLOBAL
- better clang CFLAGS checks in configure
- global vis.h hack, which made lld refuse to link to libc.so, is disabled

performance:
- single-instruction optimized math functions for aarch64, s390x, powerpc64
- fast path for ASCII in towupper/towlower
- new mostly-integer-math fma function

semantic bugs fixed:
- POSIX-format TZ dst time transitions were wrong for southern hemisphere
- regex REG_NEWLINE semantics were wrong with negated brackets
- various bugs in strptime %j, %p, %C formats
- iconv mapped some characters to legacy 8bit encodings incorrectly
- glob failed to match "/"
- UTF-8 decoder accepted invalid f4 9x xx xx code sequences
- scanf %% conversion failed to consume whitespace
- glob with GLOB_PERIOD wrongly descended into . and ..
- nftw gave incorrect base name offset when pathname ends in "/"
- functional regression in resolv.conf attempts option
- scalbn could produce wrong result due to double rounding in subnormal range
- strftime %y format wrong with negative years
- mbsnrtowcs and wcsnrtombs mishandled input limits
- minor issues with error codes for various functions

safety/consistency bugs fixed:
- stack-based buffer overflow in dns response processing
- invalid free in regexec on certain error paths
- invalid free in globfree after failed glob
- one-byte buffer overflow in legacy getpass function
- failed dlopen corrupted thread-local storage module list
- race in pthread_create with priority attributes could leave signals masked
- multithreaded set*id() functions could induce spurious EINTRs
- dl_iterate_phdr reported wrong base address in static PIE
- fd leak and wrong cancellation state after dns socket failure
- memory leaks and other issues in environment-modification functions
- read-after-free race in pthread_detach
- memmem performed single-byte over-read in short-needle code paths
- read via uninitialized pointer in gettext core
- bindtextdomain broke bindings for all other domains
- various silent undefined behavior
- getopt clobbered optopt on success

arch-specific bugs fixed:
- x32 dynamic TLS accesses crashed
- s390x was missing dlsym entry point (needed for RTLD_NEXT)
- powerpc64 ldso startup could crash depending on link order
- powerpc64 setjmp/longjmp didn't properly save/restore TOC pointer
- thumb2 setjmp/longjmp silently broke at ld-time with text not aligned
- fchown was broken on archs without SYS_fchown syscall
- fstatat was broken on mips64
- various incorrect constants in powerpc64 and mips headers

