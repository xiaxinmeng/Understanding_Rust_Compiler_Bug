
+ rustc --link-args ntwice.o play-foreign2.rs
+ ./play-foreign2
Hello world
-- C twice calling f(3)
-- C twice done w/ f(3) => 7
Rust ntwice::twice(incr2, 3): 7
-- C callback calling rtwice(incr3, 4)
---- Rust rtwice calling f(2104208)
./test-play-foreign.sh: line 14: 47675 Segmentation fault: 11  ./play-foreign2
