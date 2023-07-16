bash
> cargo clean -p lalrpop; CARGO_INCREMENTAL=0 cargo +nightly rustc --; 
   Compiling lalrpop-intern v0.14.0 (file:///Users/nmatsakis/versioned/lalrpop/lalrpop-intern)
   Compiling lalrpop-util v0.14.0 (file:///Users/nmatsakis/versioned/lalrpop/lalrpop-util)
   Compiling lalrpop-snap v0.14.0 (file:///Users/nmatsakis/versioned/lalrpop/lalrpop-snap)
   Compiling lalrpop v0.14.0 (file:///Users/nmatsakis/versioned/lalrpop/lalrpop)
    Finished dev [unoptimized + debuginfo] target(s) in 78.38 secs
> cargo clean -p lalrpop; CARGO_INCREMENTAL=0 cargo +nightly rustc --
   Compiling lalrpop v0.14.0 (file:///Users/nmatsakis/versioned/lalrpop/lalrpop)
    Finished dev [unoptimized + debuginfo] target(s) in 50.37 secs
> cargo clean -p lalrpop; CARGO_INCREMENTAL=1 cargo +nightly rustc --; 
   Compiling lalrpop-intern v0.14.0 (file:///Users/nmatsakis/versioned/lalrpop/lalrpop-intern)
   Compiling lalrpop-util v0.14.0 (file:///Users/nmatsakis/versioned/lalrpop/lalrpop-util)
   Compiling lalrpop-snap v0.14.0 (file:///Users/nmatsakis/versioned/lalrpop/lalrpop-snap)
   Compiling lalrpop v0.14.0 (file:///Users/nmatsakis/versioned/lalrpop/lalrpop)
    Finished dev [unoptimized + debuginfo] target(s) in 30.30 secs
> cargo clean -p lalrpop; CARGO_INCREMENTAL=1 cargo +nightly rustc --; 
   Compiling lalrpop v0.14.0 (file:///Users/nmatsakis/versioned/lalrpop/lalrpop)
    Finished dev [unoptimized + debuginfo] target(s) in 20.57 secs
