
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
rustup: download work dir: /home/parity/.rustup.sh/tmp/tmp-14487-0
rustup: download work dir: /home/parity/.rustup.sh/tmp/tmp-14487-1
rustup: downloading 'https://static.rust-lang.org/dist/channel-rust-stable.toml.sha256' to '/home/parity/.rustup.sh/tmp/tmp-14487-1'
rustup: moving '/home/parity/.rustup.sh/tmp/tmp-14487-1/channel-rust-stable.toml.sha256' to '/home/parity/.rustup.sh/tmp/tmp-14487-0/channel-rust-stable.toml.sha256'
rustup: cache dir: /home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4
rustup: moving '/home/parity/.rustup.sh/tmp/tmp-14487-0/channel-rust-stable.toml.sha256' to '/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml.sha256'
rustup: downloading 'https://static.rust-lang.org/dist/channel-rust-stable.toml.asc' to '/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml.asc'
rustup: downloading 'https://static.rust-lang.org/dist/channel-rust-stable.toml' to '/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml'
rustup: verifying checksums for '/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml'
rustup: sig work dir: /home/parity/.rustup.sh/tmp/tmp-14487-4
rustup: converting armored key to gpg
gpg: CRC error; 4B20BD - E455F0
rustup: command failed: gpg --no-permission-warning --dearmor /home/parity/.rustup.sh/tmp/tmp-14487-4/key.asc
rustup: signature failed for '/home/parity/.rustup.sh/dl/a7ba276c5fbbd7fb1ce4/channel-rust-stable.toml'
rustup: unable to find v2 manifest. trying v1
rustup: determining remote rust installer for 'stable'
rustup: remote rust manifest: https://static.rust-lang.org/dist/channel-rust-stable
rustup: downloading manifest for 'stable'
rustup: download work dir: /home/parity/.rustup.sh/tmp/tmp-14487-5
rustup: download work dir: /home/parity/.rustup.sh/tmp/tmp-14487-6
rustup: downloading 'https://static.rust-lang.org/dist/channel-rust-stable.sha256' to '/home/parity/.rustup.sh/tmp/tmp-14487-6'
rustup: moving '/home/parity/.rustup.sh/tmp/tmp-14487-6/channel-rust-stable.sha256' to '/home/parity/.rustup.sh/tmp/tmp-14487-5/channel-rust-stable.sha256'
rustup: cache dir: /home/parity/.rustup.sh/dl/fb2001f02d610fb9305c
rustup: moving '/home/parity/.rustup.sh/tmp/tmp-14487-5/channel-rust-stable.sha256' to '/home/parity/.rustup.sh/dl/fb2001f02d610fb9305c/channel-rust-stable.sha256'
rustup: downloading 'https://static.rust-lang.org/dist/channel-rust-stable.asc' to '/home/parity/.rustup.sh/dl/fb2001f02d610fb9305c/channel-rust-stable.asc'
rustup: downloading 'https://static.rust-lang.org/dist/channel-rust-stable' to '/home/parity/.rustup.sh/dl/fb2001f02d610fb9305c/channel-rust-stable'
rustup: verifying checksums for '/home/parity/.rustup.sh/dl/fb2001f02d610fb9305c/channel-rust-stable'
rustup: sig work dir: /home/parity/.rustup.sh/tmp/tmp-14487-9
rustup: converting armored key to gpg
gpg: CRC error; 4B20BD - E455F0
rustup: command failed: gpg --no-permission-warning --dearmor /home/parity/.rustup.sh/tmp/tmp-14487-9/key.asc
rustup: signature failed for '/home/parity/.rustup.sh/dl/fb2001f02d610fb9305c/channel-rust-stable'
rustup: leaving rustup home /home/parity/.rustup.sh
