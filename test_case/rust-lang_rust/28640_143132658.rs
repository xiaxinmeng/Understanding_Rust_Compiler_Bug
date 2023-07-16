
     -install_name name
                 Sets an internal "install path" (LC_ID_DYLIB) in a dynamic
                 library. Any clients linked against the library will record
                 that path as the way dyld should locate this library.  If
                 this option is not specified, then the -o path will be
                 used.  This option is also called -dylib_install_name for
                 compatibility.
