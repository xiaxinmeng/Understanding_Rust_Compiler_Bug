
failures:

---- [parse-fail] parse-fail/removed-syntax-uniq-mut-expr.rs stdout ----

error: unexpected compiler error or warning: '/home/ubuntu/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/parse-fail/removed-syntax-uniq-mut-expr.rs:14:25: 14:27 error: expected one of `!`, `.`, `::`, `;`, `{`, or an operator, found `42`'
status: exit code: 101
command: i686-unknown-linux-gnu/stage2/bin/rustc /home/ubuntu/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/parse-fail/removed-syntax-uniq-mut-expr.rs -L i686-unknown-linux-gnu/test/parse-fail/ --target=i686-unknown-linux-gnu -L i686-unknown-linux-gnu/test/parse-fail/removed-syntax-uniq-mut-expr.stage2-i686-unknown-linux-gnu.parse-fail.libaux -C prefer-dynamic -o i686-unknown-linux-gnu/test/parse-fail/removed-syntax-uniq-mut-expr.stage2-i686-unknown-linux-gnu --cfg rtopt -O -L i686-unknown-linux-gnu/rt -Z parse-only
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/ubuntu/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/parse-fail/removed-syntax-uniq-mut-expr.rs:14:21: 14:24 error: expected identifier, found keyword `mut`
/home/ubuntu/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/parse-fail/removed-syntax-uniq-mut-expr.rs:14     let a_box = box mut 42;
                                                                                                                                        ^~~
/home/ubuntu/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/parse-fail/removed-syntax-uniq-mut-expr.rs:14:25: 14:27 error: expected one of `!`, `.`, `::`, `;`, `{`, or an operator, found `42`
/home/ubuntu/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/parse-fail/removed-syntax-uniq-mut-expr.rs:14     let a_box = box mut 42;
                                                                                                                                            ^~

------------------------------------------

thread '[parse-fail] parse-fail/removed-syntax-uniq-mut-expr.rs' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-32-opt/build/src/compiletest/runtest.rs:1501
