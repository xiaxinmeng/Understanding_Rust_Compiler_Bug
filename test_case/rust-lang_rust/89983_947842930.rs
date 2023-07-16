
$ clang++ -static-libstdc++ main.cc
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
$ otool -L a.out
a.out:
	/usr/lib/libc++.1.dylib (compatibility version 1.0.0, current version 905.6.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1292.100.5)
