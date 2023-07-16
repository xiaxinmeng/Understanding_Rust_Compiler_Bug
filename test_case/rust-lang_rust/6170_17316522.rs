
/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr/bin/arm-apple-darwin10-llvm-g++-4.2 -E -MMD -MP -MT rt/arm-apple-darwin/arch/arm/_context.o -MF rt/arm-apple-darwin/arch/arm/_context.d ~/rust/src/rt/arch/arm/_context.S | ~/rust/llvm/x86_64-apple-darwin/Release+Asserts/bin/llvm-mc -assemble -filetype=obj -triple=arm-apple-darwin -o=rt/arm-apple-darwin/arch/arm/_context.o
/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr/bin/arm-apple-darwin10-llvm-g++-4.2 -E -MMD -MP -MT rt/arm-apple-darwin/arch/arm/ccall.o -MF rt/arm-apple-darwin/arch/arm/ccall.d ~/rust/src/rt/arch/arm/ccall.S | /Users/walterlynsdale/gplsrc/rust/llvm/x86_64-apple-darwin/Release+Asserts/bin/llvm-mc -assemble -filetype=obj -triple=arm-apple-darwin -o=rt/arm-apple-darwin/arch/arm/ccall.o
~/rust/src/rt/arch/arm/_context.S:4:7: error: unknown token in expression
.align
      ^
make: *** [rt/arm-apple-darwin/arch/arm/_context.o] Error 1
