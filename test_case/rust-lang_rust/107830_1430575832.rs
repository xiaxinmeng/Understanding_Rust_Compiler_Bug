toml
changelog-seen = 2

[llvm]
static-libstdcpp = true
download-ci-llvm = false

[build]
extended = true
docs = false
print-step-timings = true
metrics = true
submodules = false
locked-deps = true
cargo-native-static = true
configure-args = ['--enable-extended', '--disable-docs']

[rust]
codegen-units-std = 1
remap-debuginfo = true
verbose-tests = true
dist-src = false
channel = "nightly"
debuginfo-level-std = 1

[dist]
compression-formats = ["xz"]
