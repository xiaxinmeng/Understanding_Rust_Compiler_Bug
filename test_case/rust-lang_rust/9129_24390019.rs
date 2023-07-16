
% cat /tmp/issue-2216.rs
#[cfg(labelled_loop)]
pub fn main() { 'foo: loop { loop 'foo; } }

#[cfg(labelled_break)]
pub fn main() { 'foo: loop { break 'foo; } }
%  x86_64-apple-darwin/stage2/bin/rustc --cfg labelled_break  /tmp/issue-2216.rs
Bus error: 10
%  x86_64-apple-darwin/stage2/bin/rustc --cfg labelled_loop  /tmp/issue-2216.rs
Bus error: 10
% grep CONFIGURE_ARGS config.mk
CFG_CONFIGURE_ARGS   := --enable-debug --disable-optimize --enable-clang --prefix=~/opt/rust-dbg-nopt
% 
