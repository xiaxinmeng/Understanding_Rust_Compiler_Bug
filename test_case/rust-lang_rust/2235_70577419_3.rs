 sh
$ emcc -v hello.ll -o hello.js
INFO     root: (Emscripten: Running sanity checks)
WARNING: Linking two modules of different data layouts: '/Users/zen/.emscripten_cache/libc.bc' is 'e-p:32:32-i64:64-v128:32:128-n32-S128' whereas '/tmp/tmpv_yB8E/hello_0.o' is 'e-p:32:32-f64:32:64-f80:128-n8:16:32'
WARNING: Linking two modules of different target triples: /Users/zen/.emscripten_cache/libc.bc' is 'asmjs-unknown-emscripten' whereas '/tmp/tmpv_yB8E/hello_0.o' is 'i686-apple-darwin'
warning: incorrect target triple 'i686-apple-darwin' (did you use emcc/em++ on all source files and not clang directly?)
warning: unresolved symbol: _ZN2io5stdio12println_args20h0caae70b0e2eb347Iol7v0_11_0E
warning: unresolved symbol: _ZN10lang_start20h70f93b7d0a75f99atre7v0_11_0E
