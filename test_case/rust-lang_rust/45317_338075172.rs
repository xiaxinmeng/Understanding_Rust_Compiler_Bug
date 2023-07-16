
[llvm]
ccache = true
ninja = true

[build]
build = "<rust-triple>" # things in < > are replaced at build time via sed
host = ["<rust-triple>"]
target = ["<rust-triple>"]

cargo = "<cargo-bin>"
rustc = "/usr/bin/rustc"
vendor = true

full-bootstrap = false # change to true builds successfully

verbose = 0

[install]
prefix = "<prefix>"
sysconfdir = "/etc"
bindir = "<bindir>"
libdir = "<libdir>"
mandir = "<mandir>"
docdir = "<docdir>"

[rust]
codegen-units = 0
debuginfo = true

use-jemalloc = false
debug-jemalloc = false

channel = "stable"
rpath = false

codegen-tests = false
