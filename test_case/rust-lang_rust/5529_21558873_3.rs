
% ( rustc --lib /tmp/rusttmp/foo.rs && RUST_LOG=rustc::back rustc -L/tmp/rusttmp /tmp/rusttmp/bar.rs 2>&1 ) | grep 'link args:'
rust: ~"cc link args: -L/Users/fklock/opt/rust-dbg/lib/rustc/x86_64-apple-darwin/lib -m64 -o /tmp/rusttmp/bar /tmp/rusttmp/bar.o -L/Users/fklock/opt/rust-dbg/lib/rustc/x86_64-apple-darwin/lib -lstd-6c65cf4b443341b1-0.8-pre -L/tmp/rusttmp -lfoo-15fb3a718ea23983-0.0 -L/tmp/rusttmp -Wl,-no_compact_unwind -lmorestack -lrustrt -Wl,-rpath,@executable_path/../../Users/fklock/opt/rust-dbg/lib/rustc/x86_64-apple-darwin/lib -Wl,-rpath,@executable_path/. -Wl,-rpath,/Users/fklock/opt/rust-dbg/lib/rustc/x86_64-apple-darwin/lib -Wl,-rpath,/tmp/rusttmp"
% 
