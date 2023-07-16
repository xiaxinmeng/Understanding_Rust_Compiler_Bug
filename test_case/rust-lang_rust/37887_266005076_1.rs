
rustc -Z time-passes main.rs 
time: 0.000; rss: 50MB	parsing
time: 0.000; rss: 50MB	recursion limit
time: 0.000; rss: 50MB	crate injection
time: 0.000; rss: 50MB	plugin loading
time: 0.000; rss: 50MB	plugin registration
time: 0.027; rss: 71MB	expansion
time: 0.000; rss: 71MB	maybe building test harness
time: 0.000; rss: 71MB	maybe creating a macro crate
time: 0.000; rss: 71MB	checking for inline asm in case the target doesn't support it
time: 0.000; rss: 71MB	complete gated feature checking
time: 0.000; rss: 71MB	early lint checks
time: 0.000; rss: 71MB	AST validation
error[E0432]: unresolved import `libc::*`
 --> main.rs:3:9
  |
3 |     use libc::*;
  |         ^^^^^^^^ Maybe a missing `extern crate libc;`?

time: 0.002; rss: 71MB	name resolution
time: 0.000; rss: 71MB	lowering ast -> hir
time: 0.000; rss: 71MB	indexing hir
time: 0.000; rss: 71MB	attribute checking
time: 0.000; rss: 71MB	language item collection
time: 0.000; rss: 71MB	lifetime resolution
time: 0.000; rss: 71MB	looking for entry point
time: 0.000; rss: 71MB	looking for plugin registrar
time: 0.000; rss: 71MB	region resolution
time: 0.000; rss: 71MB	loop checking
time: 0.000; rss: 71MB	static item recursion checking
time: 0.000; rss: 76MB	compute_incremental_hashes_map
time: 0.000; rss: 76MB	load_dep_graph
time: 0.000; rss: 76MB	stability index
error: use of unstable library feature 'libc': use `libc` from crates.io (see issue #27783)
 --> main.rs:2:5
  |
2 |     extern crate libc;
  |     ^^^^^^^^^^^^^^^^^^
  |
  = help: add #![feature(libc)] to the crate attributes to enable

time: 0.001; rss: 76MB	stability checking
time: 0.000; rss: 76MB	type collecting
time: 0.000; rss: 76MB	variance inference
time: 0.000; rss: 76MB	impl wf inference
time: 0.008; rss: 76MB	coherence checking
time: 0.000; rss: 76MB	wf checking
time: 0.000; rss: 76MB	item-types checking
time: 0.000; rss: 76MB	item-bodies checking
time: 0.000; rss: 76MB	drop-impl checking
error: aborting due to 2 previous errors
