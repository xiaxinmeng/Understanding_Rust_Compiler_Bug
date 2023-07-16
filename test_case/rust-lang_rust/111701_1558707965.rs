diff
--- fail.txt    2023-05-23 09:20:37.218611111 +0200
+++ success.txt 2023-05-23 09:20:05.004982583 +0200
@@ -32,7 +32,7 @@ DESTDIR=""
 -m64
 -mios-simulator-version-min=7.0
 -isysroot
-/Applications/Xcode_13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk
+/Applications/Xcode_13.4.1.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator15.5.sdk
 -fembed-bitcode
 -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/llvm
 -miphoneos-version-min=10.0
@@ -42,7 +42,7 @@ DESTDIR=""
 -m64
 -mios-simulator-version-min=7.0
 -isysroot
-/Applications/Xcode_13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk
+/Applications/Xcode_13.4.1.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator15.5.sdk
 -fembed-bitcode
 -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/llvm
 -fembed-bitcode=off"
@@ -59,6 +59,6 @@ DESTDIR=""
 -m64
 -mios-simulator-version-min=7.0
 -isysroot
-/Applications/Xcode_13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk
+/Applications/Xcode_13.4.1.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator15.5.sdk
 -fembed-bitcode"
 "-DCMAKE_BUILD_TYPE=Release"
