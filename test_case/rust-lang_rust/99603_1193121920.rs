shell
$ sudo chown -R root:root src ; PATH=$(echo $PATH | tr ':' '\n' | grep -v cargo | tr '\n' ':') ; RUST_BACKTRACE=1 CI=1 SKIP_SLOW_TESTS=1 ./x.py test --stage 1 rust-analyzer ; sudo chown -R amos:amos src
