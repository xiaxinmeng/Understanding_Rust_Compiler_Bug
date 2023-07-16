
00:10:05] configure: processing command line
[00:10:05] configure: 
[00:10:05] configure: rust.debug-assertions := True
[00:10:05] configure: build.vendor         := True
[00:10:05] configure: build.submodules     := False
[00:10:05] configure: llvm.assertions      := True
[00:10:05] configure: build.locked-deps    := True
[00:10:05] configure: llvm.ccache          := sccache
[00:10:05] configure: build.openssl-static := True
[00:10:05] configure: build.build          := x86_64-unknown-linux-gnu
[00:10:05] configure: build.configure-args := ['--build=x86_64-unknown-linux-gnu', '--enable ...
[00:10:05]     for line in open(rust_dir + '/config.toml.example').read().split("\n"):
[00:10:05] IOError: [Errno 2] No such file or directory: '/checkout/obj/build/tmp/distcheck/config.toml.example'
[00:10:05] 
[00:10:05] 
[00:10:05] command did not execute successfully: "./configure" "--build=x86_64-unknown-linux-gnu" "--enable-sccache" "--disable-manage-submodules" "--enable-locked-deps" "--enable-cargo-openssl-static" "--enable-debug-assertions" "--enable-llvm-assertions" "--enable-vendor"
[00:10:05] expected success, got: exit code: 1
