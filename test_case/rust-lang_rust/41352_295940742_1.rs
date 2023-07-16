sh
$ otool -l a.out
...
Load command 15
          cmd LC_RPATH
      cmdsize 32
         path @executable_path (offset 12)
Load command 16
          cmd LC_RPATH
      cmdsize 136
         path /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/../lib/clang/8.1.0/lib/darwin (offset 12)
...
