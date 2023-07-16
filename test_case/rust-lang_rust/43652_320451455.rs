
[00:06:02] sccache: encountered fatal error
[00:06:02] sccache: error : Invalid checksum
[00:06:02] sccache:  cause: Invalid checksum
[00:06:02] make[2]: *** [lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMFastISel.cpp.o] Error 254
[00:06:02] make[2]: *** Deleting file `lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMFastISel.cpp.o'
[00:06:02] make[2]: *** Waiting for unfinished jobs....
...
$ cat obj/tmp/sccache.log
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
ERROR:sccache::server: ["ARMFastISel.cpp.o"] fatal error: Error(Io(Error { repr: Custom(Custom { kind: Other, error: StringError("Invalid checksum") }) }), State { next_error: None })
ERROR:sccache::server: [Error(Io(Error { repr: Custom(Custom { kind: Other, error: StringError("Invalid checksum") }) }), State { next_error: None })] 	ARMFastISel.cpp.o
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(InternalServerError), State { next_error: None })
$ cat /tmp/sccache.log
cat: /tmp/sccache.log: No such file or directory
