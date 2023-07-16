
[build]
target = ["x86_64-unknown-linux-gnu"] # happens to be the platform I'm building on
docs = false
extended = true
tools = ["cargo", "clippy", "src"]

[install]
prefix = "/absolute/path/to/some/existing/but/empty/dir"
