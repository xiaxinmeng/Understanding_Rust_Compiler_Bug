plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Finished release [optimized] target(s) in 2.41s
Build completed successfully in 0:00:54
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
  File "../../src/tools/publish_toolstate.py", line 87
    url = 'https://api.github.com/repos/' +
SyntaxError: invalid syntax
