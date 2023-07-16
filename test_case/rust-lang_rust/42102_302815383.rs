
$ wget https://static.rust-lang.org/dist/cargo-nightly-x86_64-unknown-linux-gnu.tar.gz
$ tar xf cargo-nightly-x86_64-unknown-linux-gnu.tar.gz 
$ ./cargo-nightly-x86_64-unknown-linux-gnu/install.sh --prefix=./usr
install: creating uninstall script at /tmp/foo/usr/lib/rustlib/uninstall.sh
install: installing component 'cargo'
install: backing up existing file at /etc/bash_completion.d/cargo
mv: cannot move '/etc/bash_completion.d/cargo' to '/etc/bash_completion.d/cargo.old': Permission denied
install: error: failed to back up /etc/bash_completion.d/cargo. see logs at '/tmp/foo/usr/lib/rustlib/install.log'
