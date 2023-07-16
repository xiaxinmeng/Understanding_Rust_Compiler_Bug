sh
NIX_PATH=nixpkgs=https://github.com/NixOS/nixpkgs-channels/archive/nixos-18.03.tar.gz \
nix-shell --pure -p rustup --run '\
    export set RUSTUP_HOME=$(pwd)/rustup-glibc-2.26; \
    rustup default nightly && \
    cargo clean && \
    cargo test -- --nocapture'
