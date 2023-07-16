
~/rust-104385$ arch
arm64
~/rust-104385$ xcode-select -p
/Applications/Xcode_14.2.app/Contents/Developer
~/rust-104385$ echo 'int main() {}' | clang -x c - -o check -arch x86_64 -mmacosx-version-min=10.7 -v 2>&1 | tail -n 1
 "/Applications/Xcode_14.2.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ld" -demangle -lto_library /Applications/Xcode_14.2.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/libLTO.dylib -no_deduplicate -dynamic -arch x86_64 -platform_version macos 10.7.0 13.1 -syslibroot /Applications/Xcode_14.2.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk -o check -lcrt1.10.6.o -L/usr/local/lib /var/folders/_x/8ymf13cn1453d8zhbz6fyr240000gn/T/--9f3a91.o -lSystem /Applications/Xcode_14.2.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/14.0.0/lib/darwin/libclang_rt.osx.a
~/rust-104385$ otool -lah check | grep -wA4 LC_VERSION_MIN_MACOSX
      cmd LC_VERSION_MIN_MACOSX
  cmdsize 16
  version 10.7
      sdk 13.1
Load command 10
