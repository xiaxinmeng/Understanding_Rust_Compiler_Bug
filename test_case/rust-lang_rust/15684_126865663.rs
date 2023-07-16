
rustup: gpg available. signatures will be verified
rustup: downloading manifest for 'stable'
rustup: downloading toolchain for 'stable'
######################################################################## 100.0%
gpg: assuming signed data in '/home/cyryl/.rustup/dl/88582fca9f8c53945b8f/rust-1.1.0-x86_64-unknown-linux-gnu.tar.gz'
gpg: Signature made Sat 20 Jun 2015 09:36:00 CEST using RSA key ID 7B3B09DC
gpg: Good signature from "Rust Language (Tag and Release Signing Key) <rust-key@rust-lang.org>" [unknown]
gpg: WARNING: This key is not certified with a trusted signature!
gpg:          There is no indication that the signature belongs to the owner.
Primary key fingerprint: 108F 6620 5EAE B0AA A8DD  5E1C 85AB 96E6 FA1B E5FE
     Subkey fingerprint: C134 66B7 E169 A085 1886  3216 5CB4 A934 7B3B 09DC
rustup: extracting installer
rustup: installing toolchain for 'stable'
[sudo] password for cyryl: 
install: uninstalling component 'rustc'
install: uninstalling component 'cargo'
install: uninstalling component 'rust-docs'
install: creating uninstall script at /usr/local/lib/rustlib/uninstall.sh
install: installing component 'rustc'
install: installing component 'cargo'
install: installing component 'rust-docs'

    Rust is ready to roll.


# vim: set noet ts=8 sts=4 sw=4:
[cyryl@localhost ~]$ rustc -V
rustc: error while loading shared libraries: librustc_driver-7d23ff90.so: cannot open shared object file: No such file or directory
[cyryl@localhost ~]$ which rustc
/usr/local/bin/rustc
[cyryl@localhost ~]$ ldd /usr/local/bin/rustc 
    linux-vdso.so.1 (0x00007ffdd93f6000)
    librustc_driver-7d23ff90.so => not found
    libstd-7d23ff90.so => not found
    libc.so.6 => /lib64/libc.so.6 (0x00007f83755fe000)
    /lib64/ld-linux-x86-64.so.2 (0x00005593d7200000)
