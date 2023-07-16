
cd /Users/logic/Projects/rustc/dev/x86_64-apple-ios/rt/compiler-rt/lib/builtins &&
/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang 
  -O3 -DNDEBUG -arch x86_64 -isysroot
  /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.11.sdk
  -Oz -Wall -fomit-frame-pointer -ffreestanding -arch x86_64 -fPIC
  -o CMakeFiles/clang_rt.hard_pic_x86_64_macho_embedded.dir/x86_64/floatdidf.c.o  
  -c /Users/logic/Projects/rustc/dev/src/compiler-rt/lib/builtins/x86_64/floatdidf.c
