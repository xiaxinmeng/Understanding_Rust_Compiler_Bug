
changelog-seen = 1

[llvm]
optimize = true
assertions = false
ninja = true

[build]
build = "x86_64-apple-darwin"
host = ["aarch64-apple-darwin"]
target = ["aarch64-apple-darwin"]
extended = true

[rust]
optimize = true
debug = false
parallel-compiler = true

[dist]
missing-tools = true
