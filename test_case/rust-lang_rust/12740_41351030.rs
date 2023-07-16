
---- [run-pass] run-pass/issue-4016.rs stdout ----

error: compilation failed!

command: x86_64-unknown-linux-gnu/stage1/bin/rustc /home/travis/build/mozilla/rust/src/test/run-pass/issue-4016.rs -L x86_64-unknown-linux-gnu/test/run-pass --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass/issue-4016.stage1-x86_64-unknown-linux-gnu.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/run-pass/issue-4016.stage1-x86_64-unknown-linux-gnu --cfg rtopt --cfg debug -L x86_64-unknown-linux-gnu/rt

stdout:

------------------------------------------

------------------------------------------

stderr:

------------------------------------------

/home/travis/build/mozilla/rust/src/test/run-pass/issue-4016.rs:16:37: 16:48 error: use of undeclared type name `json::Error`

/home/travis/build/mozilla/rust/src/test/run-pass/issue-4016.rs:16 trait JD : Decodable<json::Decoder, json::Error> { }

^~~~~~~~~~~

error: aborting due to previous error

------------------------------------------

task '[run-pass] run-pass/issue-4016.rs' failed at 'explicit failure', /home/travis/build/mozilla/rust/src/compiletest/runtest.rs:969

failures:

[run-pass] run-pass/issue-4016.rs
