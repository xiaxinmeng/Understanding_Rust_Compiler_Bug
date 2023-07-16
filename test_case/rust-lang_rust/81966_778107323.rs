
❯ cat t.c
#include <stdio.h>

int foo()
{
  printf("hello world\n");
  return 0;
}

❯ clang -target arm64-apple-ios-simulator t.c -c -o t-sim.o -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk
❯ clang -target arm64-apple-ios t.c -c -o t-ios.o -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk
❯ otool -l t-sim.o | rg 'VERSION|platform|version'
      cmd LC_BUILD_VERSION
 platform 7
❯ otool -l t-ios.o | rg 'VERSION|platform|version'
      cmd LC_VERSION_MIN_IPHONEOS
  version 7.0
