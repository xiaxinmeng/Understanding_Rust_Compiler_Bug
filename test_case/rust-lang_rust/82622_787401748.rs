
$ x.py test src/tools/cargo --test-args message_format

     Running build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/testsuite-31350880843d905f

running 11 tests
test build::wrong_message_format_option ... ok
test locate_project::message_format ... ok
test message_format::cannot_specify_two ... ok
test message_format::cargo_renders_ansi ... ok
test message_format::cargo_renders_short ... ok
test check::short_message_format ... ok
test message_format::cargo_renders ... ok
test build::message_format_json_forward_stderr ... ok
test message_format::double_json_works ... ok
test doc::doc_message_format ... ok
test doc::short_message_format ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 2108 filtered out; finished in 1.34s
