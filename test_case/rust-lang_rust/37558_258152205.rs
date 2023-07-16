
C:/Program Files/Emscripten/clang/fastcomp/build_incoming_vs2013_64/RelWithDebInfo/bin\llvm-link: c:usersuserappdatalocaltemptmpuuovrhstrlwr.c.o: error: Could not open input file: no such file or directory
C:/Program Files/Emscripten/clang/fastcomp/build_incoming_vs2013_64/RelWithDebInfo/bin\llvm-link: error loading file 'c:usersuserappdatalocaltemptmpuuovrhstrlwr.c.o'
Traceback (most recent call last):
  File "C:/Program Files/Emscripten/emscripten/1.35.0/emcc", line 1222, in <module>
    extra_files_to_link += system_libs.calculate([f for _, f in sorted(temp_files)] + extra_files_to_link, in_temp, stdout, stderr, forced=forced_stdlibs)
  File "C:\Program Files\Emscripten\emscripten\1.35.0\tools\system_libs.py", line 339, in calculate
    libfile = shared.Cache.get(name, do_create, extension=suffix)
  File "C:\Program Files\Emscripten\emscripten\1.35.0\tools\cache.py", line 41, in get
    temp = creator()
  File "C:\Program Files\Emscripten\emscripten\1.35.0\tools\system_libs.py", line 337, in do_create
    ret = create(name)
  File "C:\Program Files\Emscripten\emscripten\1.35.0\tools\system_libs.py", line 119, in create_libc
    return build_libc(libname, libc_files, args)
  File "C:\Program Files\Emscripten\emscripten\1.35.0\tools\system_libs.py", line 66, in build_libc
    shared.Building.link(o_s, in_temp(lib_filename))
  File "C:\Program Files\Emscripten\emscripten\1.35.0\tools\shared.py", line 1432, in link
    assert os.path.exists(target) and (output is None or 'Could not open input file' not in output), 'Linking error: ' + output
AssertionError: Linking error:
