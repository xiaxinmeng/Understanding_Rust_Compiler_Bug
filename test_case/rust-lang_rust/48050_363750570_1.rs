
$ sh rustup.sh --verbose --save
rustup: detecting architecture
rustup: uname -s reports: Linux
rustup: uname -m reports: x86_64
rustup: architecture is x86_64-unknown-linux-gnu
rustup: checking metadata version
rustup: writing metadata version 1
rustup: gpg available. signatures will be verified
rustup: detecting architecture
rustup: uname -s reports: Linux
rustup: uname -m reports: x86_64
rustup: architecture is x86_64-unknown-linux-gnu
rustup: dist_server: https://static.rust-lang.org
rustup: remote rust manifest: https://static.rust-lang.org/dist/channel-rust-stable.toml
rustup: downloading manifest for 'stable'
rustup: download work dir: /home/parity/.rustup.sh/tmp/tmp-12759-0
rustup: download work dir: /home/parity/.rustup.sh/tmp/tmp-12759-1
rustup: downloading 'https://static.rust-lang.org/dist/channel-rust-stable.toml.sha256' to '/home/parity/.rustup.sh/tmp/tmp-12759-1'
rustup: moving '/home/parity/.rustup.sh/tmp/tmp-12759-1/channel-rust-stable.toml.sha256' to '/home/parity/.rustup.sh/tmp/tmp-12759-0/channel-rust-stable.toml.sha256'
rustup: cache dir: /home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4
rustup: moving '/home/parity/.rustup.sh/tmp/tmp-12759-0/channel-rust-stable.toml.sha256' to '/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml.sha256'
rustup: downloading 'https://static.rust-lang.org/dist/channel-rust-stable.toml.asc' to '/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml.asc'
rustup: downloading 'https://static.rust-lang.org/dist/channel-rust-stable.toml' to '/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml'
rustup: verifying checksums for '/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml'
rustup: sig work dir: /home/parity/.rustup.sh/tmp/tmp-12759-4
rustup: converting armored key to gpg
rustup: verifying signature '/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml.asc'
gpg: directory `/home/parity/.gnupg' created
gpg: new configuration file `/home/parity/.gnupg/gpg.conf' created
gpg: WARNING: options in `/home/parity/.gnupg/gpg.conf' are not yet active during this run
gpg: keyring `/home/parity/.gnupg/pubring.gpg' created
gpg: assuming signed data in `/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml'
gpg: Signature made Thu 04 Jan 2018 04:51:17 PM CET using RSA key ID 7B3B09DC
gpg: /home/parity/.gnupg/trustdb.gpg: trustdb created
gpg: Good signature from "Rust Language (Tag and Release Signing Key) <rust-key@rust-lang.org>"
gpg: WARNING: This key is not certified with a trusted signature!
gpg:          There is no indication that the signature belongs to the owner.
Primary key fingerprint: 108F 6620 5EAE B0AA A8DD  5E1C 85AB 96E6 FA1B E5FE
     Subkey fingerprint: C134 66B7 E169 A085 1886  3216 5CB4 A934 7B3B 09DC
rustup: manifest-version: 2
rustup: detecting architecture
rustup: uname -s reports: Linux
rustup: uname -m reports: x86_64
rustup: architecture is x86_64-unknown-linux-gnu
rustup: looking for pkg.rust.target.x86_64-unknown-linux-gnu in manifest
rustup: found rust.x86_64-unknown-linux-gnu section in manifest
rustup: url: https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz
rustup: remote rust installer location: https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz
rustup: downloading toolchain for 'stable'
rustup: download work dir: /home/parity/.rustup.sh/tmp/tmp-12759-7
rustup: download work dir: /home/parity/.rustup.sh/tmp/tmp-12759-8
rustup: downloading 'https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz.sha256' to '/home/parity/.rustup.sh/tmp/tmp-12759-8'
rustup: moving '/home/parity/.rustup.sh/tmp/tmp-12759-8/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz.sha256' to '/home/parity/.rustup.sh/tmp/tmp-12759-7/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz.sha256'
rustup: cache dir: /home/parity/.rustup.sh/dl/e60a4223901d6fb24e6d
rustup: moving '/home/parity/.rustup.sh/tmp/tmp-12759-7/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz.sha256' to '/home/parity/.rustup.sh/dl/e60a4223901d6fb24e6d/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz.sha256'
rustup: downloading 'https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz.asc' to '/home/parity/.rustup.sh/dl/e60a4223901d6fb24e6d/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz.asc'
rustup: downloading 'https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz' to '/home/parity/.rustup.sh/dl/e60a4223901d6fb24e6d/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz'
######################################################################## 100.0%
rustup: verifying checksums for '/home/parity/.rustup.sh/dl/e60a4223901d6fb24e6d/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz'
sha256sum: WARNING: 1 computed checksum did NOT match
rustup: command failed: sha256sum -c /home/parity/.rustup.sh/tmp/tmp-12759-10/tmpsums
rustup: checksum failed for '/home/parity/.rustup.sh/dl/e60a4223901d6fb24e6d/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz'
rustup: leaving rustup home /home/parity/.rustup.sh

