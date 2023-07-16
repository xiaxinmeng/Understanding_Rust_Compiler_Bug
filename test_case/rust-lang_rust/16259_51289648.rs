
DYLD_LIBRARY_PATH=/Users/alex/GitHub/rust/x86_64-apple-darwin/stage2/lib:$DYLD_LIBRARY_PATH   x86_64-apple-darwin/stage2/bin/rustc --cfg stage2  -O --cfg rtopt --cfg debug  --target=x86_64-apple-darwin  -D warnings -L "x86_64-apple-darwin/rt" -L "/Users/alex/GitHub/rust/x86_64-apple-darwin/llvm/Release+Asserts/lib" -L ""  --out-dir x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib -C extra-filename=-4e7c5e5c /Users/alex/GitHub/rust/src/libhexfloat/lib.rs
info: now are following matches for libhexfloat-*.dylib libraries:
x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhexfloat-4e7c5e5c.dylib
info: now are following matches for libhexfloat-*.rlib libraries:
x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhexfloat-4e7c5e5c.rlib
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'libregex_macros-*.dylib\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'libregex_macros-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
DYLD_LIBRARY_PATH=/Users/alex/GitHub/rust/x86_64-apple-darwin/stage2/lib:$DYLD_LIBRARY_PATH   x86_64-apple-darwin/stage2/bin/rustc --cfg stage2  -O --cfg rtopt --cfg debug  --target=x86_64-apple-darwin  -D warnings -L "x86_64-apple-darwin/rt" -L "/Users/alex/GitHub/rust/x86_64-apple-darwin/llvm/Release+Asserts/lib" -L ""  --out-dir x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib -C extra-filename=-4e7c5e5c /Users/alex/GitHub/rust/src/libregex_macros/lib.rs
info: now are following matches for libregex_macros-*.dylib libraries:
x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libregex_macros-4e7c5e5c.dylib
info: now are following matches for libregex_macros-*.rlib libraries:
clang -E -E -MMD -MP -MT i686-unknown-linux-gnu/rt/arch/i386/morestack.o -MF i686-unknown-linux-gnu/rt/arch/i386/morestack.d /Users/alex/GitHub/rust/src/rt/arch/i386/morestack.S | /Users/alex/GitHub/rust/x86_64-apple-darwin/llvm/Release+Asserts/bin/llvm-mc -assemble -filetype=obj -triple=i686-unknown-linux-gnu -o=i686-unknown-linux-gnu/rt/arch/i386/morestack.o
/Users/alex/GitHub/rust/src/rt/arch/i386/morestack.S:93:18: error: unable to emit symbol attribute
 .private_extern ___morestack
                 ^
make: *** [i686-unknown-linux-gnu/rt/arch/i386/morestack.o] Error 1
