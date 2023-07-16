`sh
set -ex
tar -C /tmp/ -xzf /tmp/rust-nightly-x86_64-unknown-linux-musl.tar.gz
sh -- /tmp/rust-nightly-x86_64-unknown-linux-musl/install.sh --verbose --prefix=/opt/
PATH="/opt/bin/:{PATH:?}"
export PATH
printf '%s\n' '/lib' '/usr/local/lib' '/usr/lib' '/opt/lib' '' > /etc/ld-musl-x86_64.path
cargo -h
