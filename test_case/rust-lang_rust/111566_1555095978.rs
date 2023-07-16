plain
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: profile              := user
profile = 'user' None
changelog-seen = 2 None
download-ci-llvm = 'if-available' llvm
assertions = true llvm
ccache = 'sccache' llvm
build = 'x86_64-unknown-linux-gnu' build
locked-deps = true build
cargo-native-static = true build
cargo-native-static = true build
configure-args = ['--build=x86_64-unknown-linux-gnu', '--save-toolstates=/tmp/toolstate/toolstates.json', '--enable-sccache', '--disable-manage-submodules', '--enable-locked-deps', '--enable-cargo-native-static', '--set', 'rust.codegen-units-std=1', '--set', 'dist.compression-profile=balanced', '--dist-compression-formats=xz', '--disable-dist-src', '--release-channel=nightly', '--enable-debug-assertions', '--enable-overflow-checks', '--enable-llvm-assertions', '--set', 'rust.verify-llvm-ir', '--set', 'llvm.download-ci-llvm=if-available', '--enable-missing-tools'] build
codegen-units-std = 1 rust
debug-assertions = true rust
overflow-checks = true rust
channel = 'nightly' rust
dist-src = false rust
save-toolstates = '/tmp/toolstate/toolstates.json' rust
verify-llvm-ir = true rust
missing-tools = true dist
compression-formats = ['xz'] dist
compression-profile = 'balanced' dist
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
downloading https://static.rust-lang.org/dist/2023-04-20/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
#=#=#                                                                         
---
   Compiling clap v4.2.4
   Compiling clap_complete v4.2.2
    Finished dev [unoptimized] target(s) in 14.17s
##[endgroup]
warning: you have not made a `config.toml`
help: consider running `./x.py setup` or copying `config.example.toml` by running `cp config.example.toml config.toml`
#=#=#                                                                         

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2023-04-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt
---
####                                                                       6.9%
###############################################                           66.6%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2023-04-21/rustc-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt
warning: you have not made a `config.toml`
help: consider running `./x.py setup` or copying `config.example.toml` by running `cp config.example.toml config.toml`
Build completed successfully in 0:00:33
processor : 0
vendor_id : AuthenticAMD
cpu family : 25
---
+ /tmp/checktools.sh ../x.py
##[group]Building bootstrap
    Finished dev [unoptimized] target(s) in 0.03s
##[endgroup]
warning: you have not made a `config.toml`
help: consider running `./x.py setup` or copying `config.example.toml` by running `cp config.example.toml config.toml`
#=#=#                                                                         

#################################                                         46.1%
######################################################################## 100.0%
---
##[group]Testing stage2 rustbook src/doc/edition-guide (x86_64-unknown-linux-gnu)
Testing stage2 rustbook src/doc/edition-guide (x86_64-unknown-linux-gnu)
##[endgroup]
 finished in 1.124 seconds
warning: you have not made a `config.toml`
help: consider running `./x.py setup` or copying `config.example.toml` by running `cp config.example.toml config.toml`
Build completed successfully in 0:07:22
cat: /tmp/toolstate/toolstates.json: No such file or directory
