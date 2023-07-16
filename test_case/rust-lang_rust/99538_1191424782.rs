
changelog-seen = 2
[llvm]
download-ci-llvm = true
ninja = true
[build]
submodules = false
extended = true
tools = ["miri"]
[rust]
codegen-units = 0
debug-assertions = true
debug-assertions-std = false
debuginfo-level = 1
incremental = true
deny-warnings = false
