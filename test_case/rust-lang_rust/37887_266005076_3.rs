
rustc -Z time-passes main.rs 
time: 0.001; rss: 46MB	parsing
time: 0.000; rss: 51MB	recursion limit
time: 0.000; rss: 51MB	crate injection
time: 0.000; rss: 51MB	plugin loading
time: 0.000; rss: 51MB	plugin registration
time: 0.039; rss: 68MB	expansion
time: 0.000; rss: 68MB	maybe building test harness
time: 0.000; rss: 68MB	maybe creating a macro crate
time: 0.000; rss: 68MB	checking for inline asm in case the target doesn't support it
time: 0.000; rss: 68MB	complete gated feature checking
time: 0.000; rss: 68MB	early lint checks
time: 0.000; rss: 68MB	AST validation
error[E0432]: unresolved import `libc::*`
 --> main.rs:5:9
  |
5 |     use libc::*;
  |         ^^^^^^^^ Maybe a missing `extern crate libc;`?

time: 0.003; rss: 68MB	name resolution
time: 0.000; rss: 68MB	lowering ast -> hir
time: 0.000; rss: 68MB	indexing hir
time: 0.000; rss: 68MB	attribute checking
time: 0.000; rss: 68MB	language item collection
time: 0.000; rss: 68MB	lifetime resolution
time: 0.000; rss: 68MB	looking for entry point
time: 0.000; rss: 68MB	looking for plugin registrar
time: 0.000; rss: 68MB	region resolution
time: 0.000; rss: 68MB	loop checking
time: 0.000; rss: 68MB	static item recursion checking
time: 0.000; rss: 68MB	compute_incremental_hashes_map
time: 0.000; rss: 68MB	load_dep_graph
time: 0.000; rss: 68MB	stability index
time: 0.000; rss: 68MB	stability checking
time: 0.000; rss: 73MB	type collecting
time: 0.000; rss: 73MB	variance inference
time: 0.000; rss: 73MB	impl wf inference
time: 0.008; rss: 73MB	coherence checking
time: 0.000; rss: 73MB	wf checking
time: 0.000; rss: 73MB	item-types checking
time: 0.000; rss: 73MB	item-bodies checking
time: 0.000; rss: 73MB	drop-impl checking
error: aborting due to previous error
