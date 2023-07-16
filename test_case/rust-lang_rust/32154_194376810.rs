
[root@localhost ~]# sh rustup.sh --prefix=~/rust-test --spec=nightly -y --disable-sudo
rustup: gpg available. signatures will be verified
rustup: downloading manifest for 'nightly'
rustup: downloading toolchain for 'nightly'
######################################################################## 100.0%
gpg: assuming signed data in '/root/.rustup/dl/999afa50e1721ee68398/rust-nightly-x86_64-unknown-linux-gnu.tar.gz'
gpg: Signature made Wed 09 Mar 2016 11:22:49 AM UTC using RSA key ID 7B3B09DC
gpg: Good signature from "Rust Language (Tag and Release Signing Key) <rust-key@rust-lang.org>" [unknown]
gpg: WARNING: This key is not certified with a trusted signature!
gpg:          There is no indication that the signature belongs to the owner.
Primary key fingerprint: 108F 6620 5EAE B0AA A8DD  5E1C 85AB 96E6 FA1B E5FE
     Subkey fingerprint: C134 66B7 E169 A085 1886  3216 5CB4 A934 7B3B 09DC
rustup: installing toolchain for 'nightly'
rustup: extracting installer
install: creating uninstall script at /root/rust-test/lib/rustlib/uninstall.sh
install: installing component 'rustc'
install: installing component 'rust-std-x86_64-unknown-linux-gnu'
install: installing component 'rust-docs'
install: installing component 'cargo'

    Rust is ready to roll.

rustup.sh: line 767: ~/rust-test/lib/rustlib/channel-manifest.toml: No such file or directory
