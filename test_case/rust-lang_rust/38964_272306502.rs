
time: 0.030; rss: 38MB	parsing
time: 0.000; rss: 38MB	recursion limit
time: 0.000; rss: 38MB	crate injection
time: 0.000; rss: 38MB	plugin loading
time: 0.000; rss: 38MB	plugin registration
time: 0.150; rss: 106MB	expansion
time: 0.000; rss: 106MB	maybe building test harness
time: 0.000; rss: 106MB	maybe creating a macro crate
time: 0.000; rss: 106MB	checking for inline asm in case the target doesn't support it
time: 0.001; rss: 106MB	complete gated feature checking
time: 0.001; rss: 106MB	early lint checks
time: 0.000; rss: 106MB	AST validation
time: 0.017; rss: 111MB	name resolution
time: 0.005; rss: 115MB	lowering ast -> hir
time: 0.001; rss: 115MB	indexing hir
time: 0.000; rss: 115MB	attribute checking
time: 0.001; rss: 115MB	language item collection
time: 0.001; rss: 115MB	lifetime resolution
time: 0.000; rss: 115MB	looking for entry point
time: 0.000; rss: 115MB	looking for plugin registrar
time: 0.002; rss: 115MB	region resolution
time: 0.000; rss: 115MB	loop checking
time: 0.000; rss: 115MB	static item recursion checking
time: 0.014; rss: 115MB	compute_incremental_hashes_map
time: 0.000; rss: 115MB	load_dep_graph
time: 0.000; rss: 115MB	stability index
time: 0.002; rss: 115MB	stability checking
time: 0.003; rss: 115MB	type collecting
time: 0.000; rss: 115MB	variance inference
time: 0.000; rss: 115MB	impl wf inference
time: 0.018; rss: 128MB	coherence checking
time: 0.011; rss: 128MB	wf checking
time: 0.010; rss: 131MB	item-types checking
time: 0.286; rss: 142MB	item-bodies checking
time: 0.000; rss: 142MB	drop-impl checking
time: 0.012; rss: 142MB	const checking
time: 0.001; rss: 142MB	privacy checking
time: 0.001; rss: 142MB	intrinsic checking
time: 0.000; rss: 142MB	effect checking
time: 0.005; rss: 142MB	match checking
time: 0.002; rss: 143MB	liveness checking
time: 0.007; rss: 143MB	rvalue checking
time: 0.023; rss: 150MB	MIR dump
  time: 0.002; rss: 150MB	SimplifyCfg
  time: 0.004; rss: 150MB	QualifyAndPromoteConstants
  time: 0.006; rss: 150MB	TypeckMir
  time: 0.000; rss: 150MB	SimplifyBranches
  time: 0.001; rss: 150MB	SimplifyCfg
time: 0.014; rss: 150MB	MIR cleanup and validation
time: 0.026; rss: 151MB	borrow checking
time: 0.000; rss: 151MB	reachability checking
time: 0.001; rss: 151MB	death checking
time: 0.000; rss: 151MB	unused lib feature checking
time: 0.018; rss: 151MB	lint checking
time: 0.000; rss: 151MB	resolving dependency formats
  time: 0.000; rss: 151MB	NoLandingPads
  time: 0.001; rss: 151MB	SimplifyCfg
  time: 0.003; rss: 151MB	EraseRegions
  time: 0.001; rss: 151MB	AddCallGuards
  time: 0.011; rss: 152MB	ElaborateDrops
  time: 0.000; rss: 152MB	NoLandingPads
  time: 0.002; rss: 152MB	SimplifyCfg
  time: 0.001; rss: 152MB	InstCombine
  time: 0.000; rss: 152MB	Deaggregator
  time: 0.000; rss: 152MB	CopyPropagation
  time: 0.001; rss: 152MB	SimplifyLocals
  time: 0.001; rss: 152MB	AddCallGuards
  time: 0.000; rss: 152MB	PreTrans
time: 0.023; rss: 152MB	MIR optimisations
  time: 0.000; rss: 152MB	write metadata
  time: 0.451; rss: 200MB	translation item collection
  time: 0.051; rss: 207MB	codegen unit partitioning
time: 0.759; rss: 207MB	translation
time: 0.000; rss: 207MB	assert dep graph
time: 0.000; rss: 207MB	serialize dep graph
  time: 0.000; rss: 207MB	llvm function passes [0]
  time: 0.000; rss: 207MB	llvm module passes [0]
  time: 0.000; rss: 207MB	codegen passes [0]
  time: 0.001; rss: 207MB	codegen passes [0]
time: 0.002; rss: 207MB	LLVM passes
time: 0.000; rss: 207MB	serialize work products
time: 0.000; rss: 207MB	linking
