
Section used_crate_source
/path/to/dylib0 | /path/to/rlib0 | /path/to/rmeta0
/path/to/dylib1 | /path/to/rlib1 | /path/to/rmeta1
...
/path/to/dylibX | /path/to/rlibX | /path/to/rmetaX
-------------------------------------------------------
Section panic_runtime
None
-------------------------------------------------------
Section compiler_builtins
used_crate_source.entry0
-------------------------------------------------------
Section native_libraries
used_crate_source.entry5 | libc, libm
used_crate_source.entry9 | libc, pthread
-------------------------------------------------------
... other sections
-------------------------------------------------------
