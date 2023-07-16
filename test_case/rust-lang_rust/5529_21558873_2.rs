
% ( rustc --lib /tmp/rusttmp/foo.rs && RUST_LOG=rustc::back rustc -L/tmp/rusttmp /tmp/rusttmp/bar.rs 2>&1 ) | grep 'link args:'                                
rust: ~"cc link args: -L/home/pnkfelix/opt/rust/lib/rustc/x86_64-unknown-linux-gnu/lib -m64 -o /tmp/rusttmp/bar /tmp/rusttmp/bar.o -L/home/pnkfelix/opt/rust/lib/rustc/x86_64-unknown-linux-gnu/lib -lstd-c3ca5d77d81b46c1-0.7-pre -L/tmp/rusttmp -lfoo-68a2c114141ca-0.0 -lrustrt -lrt -lpthread -L/tmp/rusttmp -lrt -ldl -lm -lmorestack -lrustrt -Wl,-rpath,$ORIGIN/../../home/pnkfelix/opt/rust/lib/rustc/x86_64-unknown-linux-gnu/lib -Wl,-rpath,$ORIGIN/. -Wl,-rpath,/home/pnkfelix/opt/rust/lib/rustc/x86_64-unknown-linux-gnu/lib -Wl,-rpath,/tmp/rusttmp"        
% 
