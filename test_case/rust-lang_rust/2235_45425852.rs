
/Users/arcnor/emscripten-fastcomp/build/bin/llvm-nm: /tmp/tmpfTkmfj/core_0.o: Invalid CMPXCHG record.
/Users/arcnor/emscripten-fastcomp/build/bin/opt: /tmp/tmpfTkmfj/core.bc: error: Invalid CMPXCHG record
Traceback (most recent call last):
  File "/Users/arcnor/emscripten/emcc", line 1573, in <module>
    shared.Building.llvm_opt(final, link_opts)
  File "/Users/arcnor/emscripten/tools/shared.py", line 1335, in llvm_opt
    assert os.path.exists(target), 'Failed to run llvm optimizations: ' + output
AssertionError: Failed to run llvm optimizations:
