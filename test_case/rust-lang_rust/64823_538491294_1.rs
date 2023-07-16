
$ curl https://rust-lang-ci2.s3-us-west-1.amazonaws.com/rustc-builds/98951af235be5a9ac3aa0b556298b09d97d97924/rustc-dev-nightly-x86_64-unknown-linux-gnu.tar.xz | tar xJ
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  157M  100  157M    0     0  27.8M      0  0:00:05  0:00:05 --:--:-- 29.3M
$ cd rustc-dev-nightly-x86_64-unknown-linux-gnu/
$ ./install.sh --prefix=~/.rustup/toolchains/98951af235be5a9ac3aa0b556298b09d97d97924
install: creating uninstall script at /home/jistone/.rustup/toolchains/98951af235be5a9ac3aa0b556298b09d97d97924/lib/rustlib/uninstall.sh
install: installing component 'rustc-dev-x86_64-unknown-linux-gnu'
install: WARNING: failed to run ldconfig. this may happen when not installing as root. run with --verbose to see the error

    Rust is ready to develop.
