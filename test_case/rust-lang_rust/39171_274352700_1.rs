
LLVM ERROR: Function __ashlti3 has illegal integer argument
Traceback (most recent call last):
  File "/Users/rreverser/cf-repos/rust/emsdk_portable/emscripten/incoming/emcc", line 13, in <module>
    emcc.run()
  File "/Users/rreverser/cf-repos/rust/emsdk_portable/emscripten/incoming/emcc.py", line 1647, in run
    final = shared.Building.emscripten(final, append_ext=False, extra_args=extra_args)
  File "/Users/rreverser/cf-repos/rust/emsdk_portable/emscripten/incoming/tools/shared.py", line 1745, in emscripten
    call_emscripten(cmdline)
  File "/Users/rreverser/cf-repos/rust/emsdk_portable/emscripten/incoming/emscripten.py", line 1836, in _main
    temp_files.run_and_clean(lambda: main(
  File "/Users/rreverser/cf-repos/rust/emsdk_portable/emscripten/incoming/tools/tempfiles.py", line 78, in run_and_clean
    return func()
  File "/Users/rreverser/cf-repos/rust/emsdk_portable/emscripten/incoming/emscripten.py", line 1841, in <lambda>
    DEBUG=DEBUG,
  File "/Users/rreverser/cf-repos/rust/emsdk_portable/emscripten/incoming/emscripten.py", line 1742, in main
    temp_files=temp_files, DEBUG=DEBUG)
  File "/Users/rreverser/cf-repos/rust/emsdk_portable/emscripten/incoming/emscripten.py", line 91, in emscript
    funcs, metadata, mem_init = get_and_parse_backend(infile, settings, temp_files, DEBUG)
  File "/Users/rreverser/cf-repos/rust/emsdk_portable/emscripten/incoming/emscripten.py", line 160, in get_and_parse_backend
    backend_output = open(temp_js).read()
