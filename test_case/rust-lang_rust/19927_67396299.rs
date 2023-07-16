
$ curl -O https://raw.githubusercontent.com/erickt/rust/rustup/src/etc/rustup.sh
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 13759  100 13759    0     0  60259      0 --:--:-- --:--:-- --:--:-- 60346
$ sh rustup.sh
rustup: CFG_CURL             := /usr/bin/curl (7.35.0)
rustup: CFG_TAR              := /bin/tar (1.27.1)
rustup: CFG_SHASUM           := /usr/bin/shasum (5.84)
rustup: 
rustup: processing rustup.sh args
rustup: 
rustup: CFG_PREFIX           :=  
rustup: CFG_DATE             :=  
rustup: CFG_RUST_DATE        :=  
rustup: CFG_CARGO_DATE       :=  
rustup: 
rustup: validating rustup.sh args
rustup: 
rustup: host triple: x86_64-unknown-linux-gnu
rustup: Downloading https://static.rust-lang.org/dist/rust-nightly-x86_64-unknown-linux-gnu.tar.gz to ./rustup-tmp-install/rust-nightly-x86_64-unknown-linux-gnu.tar.gz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  111M  100  111M    0     0  25.4M      0  0:00:04  0:00:04 --:--:-- 26.8M
rustup: Downloading https://static.rust-lang.org/dist/rust-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   139  100   139    0     0   3987      0 --:--:-- --:--:-- --:--:--  4088
rustup: Verifying hash
rustup: Downloading https://static.rust-lang.org/cargo-dist/cargo-nightly-x86_64-unknown-linux-gnu.tar.gz to ./rustup-tmp-install/cargo-nightly-x86_64-unknown-linux-gnu.tar.gz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 4127k  100 4127k    0     0  24.8M      0 --:--:-- --:--:-- --:--:-- 25.0M
rustup: Downloading https://static.rust-lang.org/cargo-dist/cargo-nightly-x86_64-unknown-linux-gnu.tar.gz.sha256
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   141  100   141    0     0   3943      0 --:--:-- --:--:-- --:--:--  4028
rustup: Verifying hash
rustup: Extracting ./rustup-tmp-install/rust-nightly-x86_64-unknown-linux-gnu.tar.gz
/bin/tar: ./rustup-tmp-install/rust-nightly-x86_64-unknown-linux-gnu.tar.gz: Cannot open: No such file or directory
/bin/tar: Error is not recoverable: exiting now
rustup: error: failed to unpack installer
