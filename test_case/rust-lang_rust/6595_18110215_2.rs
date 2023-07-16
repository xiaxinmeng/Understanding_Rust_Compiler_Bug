
error: pretty-printed source does not typecheck
command: x86_64-unknown-linux-gnu/stage2/bin/rustc - --no-trans --lib -L x86_64-unknown-linux-gnu/test/run-pass -L x86_64-unknown-linux-gnu/test/run-pass/pub-use-xcrate.libaux -O --target=x86_64-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
<anon>:15:0: 15:27 error: multiple matching crates for `pub_use_xcrate2`
<anon>:15 extern mod pub_use_xcrate2;
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~
note: candidates:
note: path: x86_64-unknown-linux-gnu/test/run-pass/pub-use-xcrate.libaux/libpub_use_xcrate2-fdb18dcf4e777077-0.0.so
note: meta: name = "pub_use_xcrate2"
note: meta: vers = "0.0"
note: path: x86_64-unknown-linux-gnu/test/run-pass/pub-use-xcrate.libaux/libpub_use_xcrate2-4cb1dac9b641e5a-0.0.so
note: meta: name = "pub_use_xcrate2"
note: meta: vers = "0.0"
error: aborting due to previous error

------------------------------------------

rust: task failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/auto-linux/build/src/compiletest/runtest.rs:724
