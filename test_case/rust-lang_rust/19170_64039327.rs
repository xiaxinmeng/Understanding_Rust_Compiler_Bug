
$ curl -O https://raw.githubusercontent.com/erickt/rust/rustup/src/etc/rustup.sh
$ chmod +x rustup.sh
$ ./rustup.sh
rustup: CFG_CURL             := /usr/bin/curl (7.35.0)
rustup: CFG_TAR              := /bin/tar (1.27.1)
rustup: 
rustup: processing ./rustup.sh args
rustup: 
rustup: CFG_PREFIX           :=  
rustup: CFG_NIGHTLY          :=  
rustup: 
rustup: validating ./rustup.sh args
rustup: 
rustup: host triple: x86_64-unknown-linux-gnu
rustup: downloading https://static.rust-lang.org/dist/rust-nightly-x86_64-unknown-linux-gnu.tar.gz to ./rustup-tmp-install/rust-nightly-x86_64-unknown-linux-gnu.tar.gz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  105M  100  105M    0     0  43.9M      0  0:00:02  0:00:02 --:--:-- 43.9M
rustup: Extracting ./rustup-tmp-install/rust-nightly-x86_64-unknown-linux-gnu.tar.gz
/bin/tar: ./rustup-tmp-install/rust-nightly-x86_64-unknown-linux-gnu.tar.gz: Cannot open: No such file or directory
/bin/tar: Error is not recoverable: exiting now
rustup: error: failed to unpack installer
