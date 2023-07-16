
landin:~ lkuper$ export RUST_LOG=foo::a,foo::b=0; ./foo
rust: ~"error from module a"
rust: ~"warn from module a"
rust: ~"info from module a"
rust: ~"debug from module a"
landin:~ lkuper$
