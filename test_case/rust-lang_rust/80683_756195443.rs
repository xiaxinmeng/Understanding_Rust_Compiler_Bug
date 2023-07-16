
$ x.py install
...
   Compiling tempfile v3.1.0
   Compiling regex v1.3.9
   Compiling rustdoc v0.0.0 (/home/vext01/source/rust/src/librustdoc)
   Compiling rustdoc-tool v0.0.0 (/home/vext01/source/rust/src/tools/rustdoc)
    Finished release [optimized] target(s) in 1m 32s
Dist rustc-1.51.0-dev-x86_64-unknown-linux-gnu
        finished in 40.785 seconds
Install rustc stage2 (Some(TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }))
install: creating uninstall script at /home/vext01/source/rust/build/tmp/empty_dir/build/ykrustc-stage2-latest/lib/rustlib/uninstall.sh
install: installing component 'rustc'

    rustc installed.

warning: x.py has made several changes recently you may want to look at
help: consider looking at the changes in `src/bootstrap/CHANGELOG.md`
note: to silence this warning, add `changelog-seen = 2` at the top of `config.toml`
note: this message was printed twice to make it more likely to be seen
Build completed successfully in 0:28:40
$ find build/ykrustc-stage2-latest/
build/ykrustc-stage2-latest/
$
