
[00:48:07] ---- [ui] ui/chalkify/lower_env1.rs stdout ----
[00:48:07] 	diff of stderr:
[00:48:07] 
[00:48:07] 14	LL | #[rustc_dump_env_program_clauses] //~ ERROR program clause dump
[00:48:07] 15	   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:48:07] 16	   |
[00:48:07] +	   = note: Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).
[00:48:07] 17	   = note: Implemented(Self: Bar) :- FromEnv(Self: Bar).
[00:48:07] 18	   = note: FromEnv(Self: Bar) :- FromEnv(Self: Bar).
[00:48:07] 19	   = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).
[00:48:07] 
[00:48:07] -	   = note: Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).
[00:48:07] 21	   = note: Implemented(Self: Foo) :- FromEnv(Self: Foo).
[00:48:07] 22	
[00:48:07] 23	error: aborting due to 2 previous errors
