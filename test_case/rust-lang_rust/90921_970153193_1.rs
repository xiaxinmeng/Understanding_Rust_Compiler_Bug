 
$ python3 ./x.py clean
...
Build completed successfully in 0:00:00

$ python3 ./x.py test src/test/run-make-fulldeps --test-args long-linker-command-lines
...
   Compiling proc-macro2 v1.0.24
...
running 2 tests
test [run-make] run-make-fulldeps/long-linker-command-lines-cmd-exe ... ok
test [run-make] run-make-fulldeps/long-linker-command-lines ... FAILED
...
------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'status: exit status: 127
stdout:

stderr:
/home/abuild/rpmbuild/BUILD/rustc-1.56.1-src/build/x86_64-unknown-linux-gnu/stage1/bin/rustc: error while loading shared libraries: librustc_driver-0cce5dc831adf0c6.so: cannot open shared object file:...
', foo.rs:76:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
make: *** [Makefile:5: all] Error 101

------------------------------------------
...
