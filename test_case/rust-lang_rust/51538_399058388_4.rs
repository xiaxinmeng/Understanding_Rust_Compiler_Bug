\ntrait Foo {\n    fn bar(&self);\n}\n\nfn some_func<T: Foo>(foo: T) {\n    foo.bar(); // we can now use this method since i32 implements the\n               // Foo trait\n}\n\n// we implk" "--android-cross-path" "" "--color" "always"
[00:45:53] 
[00:45:53] 
[00:45:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:53] Build completed unsuccessfully in 0:02:02
[00:45:53] Build completed unsuccessfully in 0:02:02
[00:45:53] Makefile:58: recipe for target 'check' failed
[00:45:53] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:007e2d1e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
144560 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
144556 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
137996 ./obj/build/bootstrap/debug/incremental
123424 ./obj/build/bootstrap/debug/incremental/bootstrap-3ro763qrbnb6h
123420 ./obj/build/bootstrap/debug/incremental/bootstrap-3ro763qrbnb6h/s-f26t7krk9t-6a80so-xcsc5o6cxe7j
107596 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
103648 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
102844 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
102840 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
