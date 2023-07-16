
$ brew tap SergioBenitez/osxct
$ brew install x86_64-unknown-linux-gnu

# .cargo/config.toml
[build]
target = "x86_64-unknown-linux-gnu"

[target.x86_64-unknown-linux-gnu]
linker = "/usr/local/bin/x86_64-unknown-linux-gnu-gcc"

