
ld: archive has no table of contents file '/Users/benanderson/workspace/rust/test/target/deps/libtestlib-79bbf85880b0bfea.rlib' for architecture x86_64
0  0x10573ab51  __assert_rtn + 144
1  0x10578b8ce  ld::tool::InputFiles::waitForInputFiles() + 0
2  0x7fff894ee2fc  _pthread_body + 131
3  0x7fff894ee279  _pthread_body + 0
A linker snapshot was created at:
    (null)
ld: Assertion failed: (slot < (int)files.size()), function parseWorkerThread, file /SourceCache/ld64/ld64-241.9/src/ld/InputFiles.cpp, line 920.
clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error
Could not compile `hello_world`.
