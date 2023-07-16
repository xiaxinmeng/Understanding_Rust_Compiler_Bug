
...
...
install: verifying installed binaries are executable

    Rust is ready to roll.

rustup: Extracting cargo-nightly-x86_64-apple-darwin.tar.gz
x cargo-nightly-x86_64-apple-darwin/
x cargo-nightly-x86_64-apple-darwin/bin/
x cargo-nightly-x86_64-apple-darwin/etc/
x cargo-nightly-x86_64-apple-darwin/install.sh
x cargo-nightly-x86_64-apple-darwin/lib/
x cargo-nightly-x86_64-apple-darwin/LICENSE-APACHE
x cargo-nightly-x86_64-apple-darwin/LICENSE-MIT
x cargo-nightly-x86_64-apple-darwin/LICENSE-THIRD-PARTY
x cargo-nightly-x86_64-apple-darwin/README.md
x cargo-nightly-x86_64-apple-darwin/share/
x cargo-nightly-x86_64-apple-darwin/share/doc/
x cargo-nightly-x86_64-apple-darwin/share/man/
x cargo-nightly-x86_64-apple-darwin/share/zsh/
x cargo-nightly-x86_64-apple-darwin/share/zsh/site-functions/
x cargo-nightly-x86_64-apple-darwin/share/zsh/site-functions/_cargo
x cargo-nightly-x86_64-apple-darwin/share/man/man1/
x cargo-nightly-x86_64-apple-darwin/share/man/man1/cargo.1
x cargo-nightly-x86_64-apple-darwin/share/doc/cargo/
x cargo-nightly-x86_64-apple-darwin/share/doc/cargo/LICENSE-APACHE
x cargo-nightly-x86_64-apple-darwin/share/doc/cargo/LICENSE-MIT
x cargo-nightly-x86_64-apple-darwin/share/doc/cargo/LICENSE-THIRD-PARTY
x cargo-nightly-x86_64-apple-darwin/share/doc/cargo/README.md
x cargo-nightly-x86_64-apple-darwin/lib/cargo/
x cargo-nightly-x86_64-apple-darwin/lib/cargo/manifest.in
x cargo-nightly-x86_64-apple-darwin/etc/bash_completion.d/
x cargo-nightly-x86_64-apple-darwin/etc/bash_completion.d/cargo
x cargo-nightly-x86_64-apple-darwin/bin/cargo
install: looking for install programs
install: found mkdir
install: found printf
install: found cut
install: found grep
install: found uname
install: found tr
install: found sed
install: 
install: processing ./rustup-tmp-install/cargo-nightly-x86_64-apple-darwin/install.sh args
install: 
install: CFG_DESTDIR          :=  
install: CFG_PREFIX           := /usr/local 
install: CFG_LIBDIR           := /usr/local/lib 
install: CFG_MANDIR           := /usr/local/share/man 
install: 
install: validating ./rustup-tmp-install/cargo-nightly-x86_64-apple-darwin/install.sh args
install: 
install: verifying platform can run binaries
./rustup-tmp-install/cargo-nightly-x86_64-apple-darwin/install.sh: line 314:  2742 Segmentation fault: 11  "${CFG_SRC_DIR}/bin/cargo" -V 2> /dev/null
install: error: can't execute cargo binary on this platform
rustup: error: failed to install Rust
