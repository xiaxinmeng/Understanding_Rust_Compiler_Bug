
error: pretty-printed source does not typecheck
command: x86_64-unknown-linux-gnu/stage2/bin/rustc - --no-trans --lib -L x86_64-unknown-linux-gnu/test/run-pass -L x86_64-unknown-linux-gnu/test/run-pass/issue-2631-b.libaux -O --target=x86_64-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
<anon>:14:0: 14:15 error: multiple matching crates for `req`
<anon>:14 extern mod req;
          ^~~~~~~~~~~~~~~
note: candidates:
note: path: x86_64-unknown-linux-gnu/test/run-pass/issue-2631-b.libaux/libreq-bef4e145ee4b741-0.0.so
note: meta: name = "req"
note: meta: vers = "0.0"
note: path: x86_64-unknown-linux-gnu/test/run-pass/issue-2631-b.libaux/libreq-94839cbfe144198-0.0.so
note: meta: name = "req"
note: meta: vers = "0.0"
error: aborting due to previous error

------------------------------------------
