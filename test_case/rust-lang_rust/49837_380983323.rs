
error: program clause dump
  --> $DIR/lower_env1.rs:18:1
   |
LL | #[rustc_dump_env_program_clauses] //~ ERROR program clause dump
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).
   = note: Implemented(Self: Bar) :- FromEnv(Self: Bar).
   = note: FromEnv(Self: Bar) :- FromEnv(Self: Bar).
   = note: Implemented(Self: Foo) :- FromEnv(Self: Foo).

error: aborting due to previous error
