
% ./x86_64-apple-darwin/stage1/bin/rustc  /tmp/asm-play.rs && ./asm-play 
/tmp/asm-play.rs:21:9: 21:14 warning: value assigned to `x` is never read, #[warn(dead_assignment)] on by default
/tmp/asm-play.rs:21     let mut x: int = 17;
                            ^~~~~
eval input-2, x: 17
%
