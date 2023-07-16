
[01:35:06]   = note: Unsupported:   call void @_ZN28float_int_invalid_const_cast5force17haaefff15b0074e85E(i128 0)
[01:35:06]           LLVM ERROR: Instruction not yet supported for integer types larger than 64 bits
[01:35:06]           Traceback (most recent call last):
[01:35:06]             File "/emsdk-portable/emscripten/1.37.13//emcc", line 13, in <module>
[01:35:06]               emcc.run()
[01:35:06]             File "/emsdk-portable/emscripten/1.37.13/emcc.py", line 1526, in run
[01:35:06]               final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
