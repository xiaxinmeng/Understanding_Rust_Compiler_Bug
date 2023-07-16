
[llvm]
link-shared = true

[build]
host = ["x86_64-unknown-linux-gnu"]
target = ["armv7-unknown-androideabi", "wasm32-unknown-unknown"]



python = "python2.7"
extended = true
sanitizers = false

#tools = ["cargo", "analysis", "rls", "rustfmt", "clippy-driver" ]

[install]
prefix = "/data/data/com.termux/files/usr"
sysconfdir = "etc"

[rust]
optimize = true
debug = false
codegen-units = 0
jemalloc = false
channel = "stable"
rpath = false

[target.x86_64-unknown-linux-gnu]
llvm-config = "/usr/bin/llvm-config-8"

[target.armv7-unknown-androideabi]
android-ndk = "@TERMUX_STANDALONE_TOOLCHAIN@"
llvm-config = "/data/data/com.termux/files/usr/bin/llvm-config"

[dist]
src-tarball = false
#missing-tools = true
