
cd /Users/logic/Projects/rustc/dev/aarch64-apple-ios/rt/compiler-rt/lib/builtins &&
/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang
  -O3 -DNDEBUG -arch arm64    -isysroot
  /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS9.3.sdk
  -miphoneos-version-min=6.0 -fPIC -O3 -fvisibility=hidden -DVISIBILITY_HIDDEN -Wall -fomit-frame-pointer -arch arm64
  -o CMakeFiles/clang_rt.builtins_arm64_ios.dir/floatdidf.c.o 
  -c /Users/logic/Projects/rustc/dev/src/compiler-rt/lib/builtins/floatdidf.c
