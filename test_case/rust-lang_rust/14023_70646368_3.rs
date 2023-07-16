
...
make[2]: Entering directory '/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support'
/home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/autoconf/mkinstalldirs /home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts > /dev/null
/usr/bin/date > /home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/.dir
llvm[2]: Compiling APFloat.cpp for Release+Asserts build
if  ccache clang++ -Qunused-arguments -I/home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/include -I/home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/lib/Support -I/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/include -I/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support  -D_DEBUG -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -O3 -fomit-frame-pointer -stdlib=libc++ -std=c++11 -fvisibility-inlines-hidden -fno-exceptions -fno-rtti -fPIC -ffunction-sections -fdata-sections -Wcast-qual -march=native -O2 -pipe -fstack-protector-strong --param=ssp-buffer-size=4 -D_FORTIFY_SOURCE=2 -Wno-trigraphs   -D_FORTIFY_SOURCE=2 -D_FORTIFY_SOURCE=2 -D_FORTIFY_SOURCE=2  -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcovered-switch-default -Wno-uninitialized -Wno-missing-field-initializers -c -MMD -MP -MF "/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.d.tmp" -MT "/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.o" -MT "/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.d" /home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/lib/Support/APFloat.cpp -o /home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.o ; \
        then /usr/bin/mv -f "/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.d.tmp" "/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.d"; else /usr/bin/rm "/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.d.tmp"; exit 1; fi
In file included from /home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/lib/Support/APFloat.cpp:15:
In file included from /home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/include/llvm/ADT/APFloat.h:20:
In file included from /home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/include/llvm/ADT/APInt.h:19:
In file included from /home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/include/llvm/ADT/ArrayRef.h:14:
/home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/include/llvm/ADT/STLExtras.h:21:10: fatal error: 
      'cstddef' file not found
#include <cstddef> // for std::size_t
         ^
1 error generated.
/usr/bin/rm: cannot remove ‘/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.d.tmp’: No such file or directory
/home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/Makefile.rules:1514: recipe for target '/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.o' failed
make[2]: *** [/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.o] Error 1
make[2]: Leaving directory '/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/lib/Support'
/home/emacs/build/rust-git/makepkg/rust-git/src/rust/src/llvm/Makefile.rules:873: recipe for target 'all' failed
make[1]: *** [all] Error 1
make[1]: Leaving directory '/home/emacs/build/rust-git/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm'
/tmp/makepkg/rust-git/src/rust/mk/llvm.mk:60: recipe for target '/tmp/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/bin/llvm-config' failed
make: *** [/tmp/makepkg/rust-git/src/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/bin/llvm-config] Error 2
==> ERROR: A failure occurred in build().
    Aborting...

real    1m7.847s
user    0m42.023s
sys 0m10.023s

