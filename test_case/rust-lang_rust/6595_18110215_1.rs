
error: pretty-printed source does not typecheck
command: x86_64-unknown-linux-gnu/stage2/bin/rustc - --no-trans --lib -L x86_64-unknown-linux-gnu/test/run-pass -L x86_64-unknown-linux-gnu/test/run-pass/issue2378c.libaux -O --target=x86_64-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
<anon>:16:0: 16:22 error: multiple matching crates for `issue2378b`
<anon>:16 extern mod issue2378b;
          ^~~~~~~~~~~~~~~~~~~~~~
note: candidates:
note: path: x86_64-unknown-linux-gnu/test/run-pass/issue2378c.libaux/libissue2378b-fdb18dcf4e777077-0.0.so
note: meta: name = "issue2378b"
note: meta: vers = "0.0"
note: path: x86_64-unknown-linux-gnu/test/run-pass/issue2378c.libaux/libissue2378b-4cb1dac9b641e5a-0.0.so
note: meta: name = "issue2378b"
note: meta: vers = "0.0"
error: aborting due to previous error

------------------------------------------
