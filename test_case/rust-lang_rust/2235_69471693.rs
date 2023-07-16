
Value:   %28 = call fastcc { i8, [0 x i8], [0 x i8] } @_ZN3fmt5write20h2c56fdda0b308d94DFAE({ i8*, void (i8*)** }* noalias nocapture dereferenceable(8) %arg.i, %"struct.core::fmt::Arguments[#3]"* noalias nocapture readonly dereferenceable(24) %__args31), !noalias !22
LLVM ERROR: Unrecognized struct value
Traceback (most recent call last):
  File "/Users/chris/Downloads/emsdk_portable/emscripten/incoming/emcc", line 1259, in <module>
    shared.Building.llvm_opt(final, link_opts)
  File "/Users/chris/Downloads/emsdk_portable/emscripten/incoming/tools/shared.py", line 1401, in llvm_opt
    assert os.path.exists(target), 'Failed to run llvm optimizations: ' + output
AssertionError: Failed to run llvm optimizations:
