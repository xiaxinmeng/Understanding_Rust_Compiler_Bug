sh
$ clang++ -fsanitize=address 1.cpp
$ otool -L a.out
a.out:
	/usr/lib/libc++.1.dylib (compatibility version 1.0.0, current version 307.5.0)
	@rpath/libclang_rt.asan_osx_dynamic.dylib (compatibility version 0.0.0, current version 0.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1238.51.1)
