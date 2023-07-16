
wesley@endurance:~/code/rust/rustc-serialize> RUSTFLAGS="-Zself-profile" cargo build
   Compiling rustc-serialize v0.3.24 (file:///home/wesley/code/rust/rustc-serialize)
Self profiling results for rustc_serialize:
Parsing         		 27             
Expansion       		 132            
TypeChecking    		 2199           
	1597268 hits 1666449 queries
BorrowChecking  		 150            
	10064 hits 14436 queries
Codegen         		 2098           
	44011 hits 54580 queries
Linking         		 17             
	4724 hits 5660 queries
Other           		 601            
	2389409 hits 2461645 queries

Optimization level: No
Incremental: on
    Finished dev [unoptimized + debuginfo] target(s) in 5.36 secs

