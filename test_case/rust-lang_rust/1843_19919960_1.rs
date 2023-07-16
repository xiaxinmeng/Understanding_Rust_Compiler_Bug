
+ rustc --link-args ntwice.o play-foreign.rs
+ ./play-foreign
Hello world
-- C twice calling f(3)
-- C twice done w/ f(3) => 7
Rust ntwice::twice(incr2, 3): 7
-- C callback calling rtwice(incr3, 4)
---- Rust rtwice calling f(4)
------ C Enter incr3(f, 120493568)
------ C Finis incr3(f, 120493568) => 120493571
------ C Enter incr3(f, 1283)
------ C Finis incr3(f, 1283) => 1286
---- Rust rtwice calling f(4) => 1286
-- C callback done w/ rtwice(incr3, 4) => 1286
Rust ntwice::callback(rtwice, 4): 1286
