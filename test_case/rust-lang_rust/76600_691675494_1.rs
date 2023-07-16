
lib/libc/gen/pthread_atfork.c                   1.13,1.14
libexec/ld.elf_so/rtld.c                        1.204,1.205
libexec/ld.elf_so/rtld.h                        1.139,1.140
libexec/ld.elf_so/symbols.map                   1.3,1.4

        Introduce intermediate locking for fork, so that the dynamic
        linker is in a consistent state.
        [chs, ticket #907]
