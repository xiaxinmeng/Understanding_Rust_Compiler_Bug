
me@my-Mac ~ % arch --x86_64 \
sh <(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs) \
--default-host aarch64-apple-darwin \
--default-toolchain nightly
arch: posix_spawnp: sh: Bad CPU type in executable
me@my-Mac ~ % curl: (23) Failed writing body (0 != 710)
