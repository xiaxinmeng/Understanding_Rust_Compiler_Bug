
foo.rs:2:9: 2:14 error: Illegal anonymous lifetime: anonymous lifetimes are not permitted here
foo.rs:2 struct S(&fn());
                  ^~~~~
foo.rs:3:14: 3:16 error: Illegal anonymous lifetime: only 'static is allowed here
foo.rs:3 pub static C: S = S(f);
                       ^~
