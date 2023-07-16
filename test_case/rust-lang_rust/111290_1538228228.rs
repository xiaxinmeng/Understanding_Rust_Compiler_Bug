`
$ rustup toolchain install beta
info: syncing channel updates for 'beta-x86_64-unknown-linux-gnu'
info: latest update on 2023-05-08, rust version 1.70.0-beta.4 (2013813b6 2023-05-07)
error: some components unavailable for download for channel 'beta': 'clippy' for target 'x86_64-unknown-linux-gnu', 'rustfmt' for target 'x86_64-unknown-linux-gnu'If you don't need the components, you could try a minimal installation with:

    rustup toolchain add beta --profile minimal
