
rustc -Z time-passes main.rs 
time: 0.000; rss: 46MB	parsing
time: 0.000; rss: 51MB	recursion limit
time: 0.000; rss: 51MB	crate injection
time: 0.000; rss: 51MB	plugin loading
time: 0.000; rss: 51MB	plugin registration
time: 0.038; rss: 68MB	expansion
time: 0.000; rss: 68MB	maybe building test harness
time: 0.000; rss: 68MB	maybe creating a macro crate
time: 0.000; rss: 68MB	checking for inline asm in case the target doesn't support it
time: 0.000; rss: 68MB	complete gated feature checking
time: 0.000; rss: 68MB	early lint checks
time: 0.000; rss: 68MB	AST validation
time: 0.005; rss: 68MB	name resolution
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
time: 0.000; rss: 72MB	load_dep_graph
time: 0.000; rss: 72MB	stability index
time: 0.000; rss: 72MB	stability checking
time: 0.000; rss: 72MB	type collecting
time: 0.000; rss: 72MB	variance inference
time: 0.000; rss: 72MB	impl wf inference
time: 0.008; rss: 72MB	coherence checking
time: 0.000; rss: 72MB	wf checking
time: 0.000; rss: 72MB	item-types checking
time: 0.000; rss: 72MB	item-bodies checking
time: 0.000; rss: 72MB	drop-impl checking
time: 0.000; rss: 72MB	const checking
time: 0.000; rss: 72MB	privacy checking
time: 0.000; rss: 72MB	intrinsic checking
time: 0.000; rss: 72MB	effect checking
time: 0.000; rss: 72MB	match checking
time: 0.000; rss: 72MB	liveness checking
time: 0.000; rss: 72MB	rvalue checking
time: 0.000; rss: 72MB	MIR dump
  time: 0.000; rss: 72MB	SimplifyCfg
  time: 0.000; rss: 72MB	QualifyAndPromoteConstants
  time: 0.000; rss: 72MB	TypeckMir
  time: 0.000; rss: 72MB	SimplifyBranches
  time: 0.000; rss: 72MB	SimplifyCfg
time: 0.000; rss: 72MB	MIR cleanup and validation
time: 0.000; rss: 72MB	borrow checking
time: 0.000; rss: 72MB	reachability checking
time: 0.000; rss: 72MB	death checking
time: 0.000; rss: 72MB	unused lib feature checking
warning: unused import: `libc::*;`, #[warn(unused_imports)] on by default
 --> main.rs:6:9
  |
6 |     use libc::*;
  |         ^^^^^^^^

time: 0.008; rss: 72MB	lint checking
time: 0.002; rss: 72MB	resolving dependency formats
  time: 0.000; rss: 72MB	NoLandingPads
  time: 0.000; rss: 72MB	SimplifyCfg
  time: 0.000; rss: 72MB	EraseRegions
  time: 0.000; rss: 72MB	AddCallGuards
  time: 0.000; rss: 72MB	ElaborateDrops
  time: 0.000; rss: 72MB	NoLandingPads
  time: 0.000; rss: 72MB	SimplifyCfg
  time: 0.000; rss: 72MB	InstCombine
  time: 0.000; rss: 72MB	Deaggregator
  time: 0.000; rss: 72MB	CopyPropagation
  time: 0.000; rss: 72MB	SimplifyLocals
  time: 0.000; rss: 72MB	AddCallGuards
  time: 0.000; rss: 72MB	PreTrans
time: 0.000; rss: 72MB	MIR optimisations
  time: 0.000; rss: 72MB	write metadata
  time: 0.000; rss: 72MB	translation item collection
  time: 0.000; rss: 72MB	codegen unit partitioning
  time: 0.000; rss: 79MB	internalize symbols
time: 0.096; rss: 79MB	translation
time: 0.000; rss: 79MB	assert dep graph
time: 0.000; rss: 79MB	serialize dep graph
  time: 0.000; rss: 79MB	llvm function passes [0]
  time: 0.000; rss: 79MB	llvm module passes [0]
  time: 0.003; rss: 79MB	codegen passes [0]
  time: 0.000; rss: 79MB	codegen passes [0]
time: 0.006; rss: 79MB	LLVM passes
time: 0.000; rss: 79MB	serialize work products
  time: 0.197; rss: 79MB	running linker
time: 0.198; rss: 79MB	linking
