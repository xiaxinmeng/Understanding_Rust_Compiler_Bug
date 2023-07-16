
#![feature(trace_macros)]
trace_macros!(true);

macro_rules! nest {
  ($n:expr) => (id!($n))
}

macro_rules! id {
  ($n:tt) => ($n)
}

fn main() {
   nest! { S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(S(0)))))))))))))))))))))))))) };
}
