
$ clang -target riscv64-unknown-openbsd -x c -dM -E - </dev/null | grep _CHAR_
#define __CHAR_BIT__ 8
#define __CHAR_UNSIGNED__ 1
#define __CLANG_ATOMIC_CHAR_LOCK_FREE 2
#define __GCC_ATOMIC_CHAR_LOCK_FREE 2

$ clang -target x86_64-unknown-openbsd -x c -dM -E - </dev/null | grep _CHAR_
#define __CHAR_BIT__ 8
#define __CLANG_ATOMIC_CHAR_LOCK_FREE 2
#define __GCC_ATOMIC_CHAR_LOCK_FREE 2
