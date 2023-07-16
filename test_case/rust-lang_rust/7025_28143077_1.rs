
$ rustc 7025.rs --cfg 'target_os="win32"'
7025.rs:7:0: 7:11 error: duplicate definition of value `foo`
7025.rs:7 fn foo() {}
          ^~~~~~~~~~~
7025.rs:4:0: 4:11 note: first definition of value `foo` here
7025.rs:4 fn foo() {}
          ^~~~~~~~~~~
error: aborting due to previous error
task 'rustc' failed at 'explicit failure', /home/huon/rust/src/libsyntax/diagnostic.rs:101
task '<main>' failed at 'explicit failure', /home/huon/rust/src/librustc/lib.rs:397
