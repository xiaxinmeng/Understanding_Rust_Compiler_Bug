bash
# start with a vanilla system image
podman run --rm -it -v ~/.ssh/:/root/.ssh archlinux

# install build dependencies
pacman -Sy base-devel pkg-config python curl openssl git make cmake openssh
git clone https://github.com/rust-lang/rust.git
cd rust
./x.py setup
# choose c
./x.py test --stage 1 src/test/ui
# Couldn't find required command: ninja
# You should install ninja, or set `ninja=false` in config.toml in the `[llvm]` section.

# ... edit config.toml
./x.py test --stage 1 src/test/ui

# works!
