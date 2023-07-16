
---:~ Guest$ curl https://sh.rustup.rs -sSf | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust programming 
language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to Cargo's bin 
directory, located at:

  /Users/Guest/.cargo/bin

This path will then be added to your PATH environment variable by modifying the
profile file located at:

  /Users/Guest/.profile

You can uninstall at any time with rustup self uninstall and these changes will
be reverted.

Current installation options:

   default host triple: x86_64-apple-darwin
     default toolchain: stable
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation


info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2018-06-21, rust version 1.27.0 (3eda71b00 2018-06-19)
info: downloading component 'rustc'
 56.4 MiB /  56.4 MiB (100 %)  12.4 MiB/s ETA:   0 s                
info: downloading component 'rust-std'
 44.8 MiB /  44.8 MiB (100 %)  13.1 MiB/s ETA:   0 s                
info: downloading component 'cargo'
info: downloading component 'rust-docs'
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: default toolchain set to 'stable'

  stable installed - rustc 1.27.0 (3eda71b00 2018-06-19)


Rust is installed now. Great!

To get started you need Cargo's bin directory ($HOME/.cargo/bin) in your PATH 
environment variable. Next time you log in this will be done automatically.

To configure your current shell run source $HOME/.cargo/env
