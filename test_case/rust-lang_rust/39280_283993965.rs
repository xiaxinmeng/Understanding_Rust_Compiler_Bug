
time: 0.392; rss: 97MB	parsing
time: 0.000; rss: 97MB	recursion limit
time: 0.000; rss: 97MB	crate injection
time: 0.001; rss: 98MB	plugin loading
time: 0.000; rss: 98MB	plugin registration
time: 4.876; rss: 650MB	expansion
time: 0.000; rss: 650MB	maybe building test harness
time: 0.038; rss: 650MB	maybe creating a macro crate
time: 0.000; rss: 650MB	checking for inline asm in case the target doesn't support it
time: 0.188; rss: 650MB	early lint checks
time: 0.043; rss: 650MB	AST validation
time: 0.947; rss: 868MB	name resolution
time: 0.111; rss: 868MB	complete gated feature checking
time: 0.684; rss: 1103MB	lowering ast -> hir
time: 0.107; rss: 1017MB	indexing hir
time: 0.039; rss: 1017MB	attribute checking
time: 0.026; rss: 806MB	language item collection
time: 0.129; rss: 815MB	lifetime resolution
time: 0.000; rss: 815MB	looking for entry point
time: 0.003; rss: 815MB	looking for plugin registrar
time: 0.452; rss: 898MB	region resolution
time: 0.075; rss: 898MB	loop checking
time: 0.053; rss: 898MB	static item recursion checking
time: 1.298; rss: 948MB	compute_incremental_hashes_map
time: 0.000; rss: 948MB	load_dep_graph
time: 0.138; rss: 963MB	stability index
time: 0.331; rss: 990MB	stability checking
time: 0.548; rss: 1063MB	type collecting
time: 0.208; rss: 1067MB	variance inference
time: 0.220; rss: 1124MB	impl wf inference
time: 0.572; rss: 1188MB	coherence checking
time: 0.726; rss: 1193MB	wf checking
time: 2.519; rss: 1262MB	item-types checking
time: 15.323; rss: 1806MB	item-bodies checking
time: 0.000; rss: 1806MB	drop-impl checking
time: 1.276; rss: 1906MB	const checking
time: 1.114; rss: 1906MB	privacy checking
time: 0.283; rss: 1911MB	intrinsic checking
time: 0.114; rss: 1911MB	effect checking
time: 0.417; rss: 1917MB	match checking
time: 0.196; rss: 1919MB	liveness checking
time: 0.928; rss: 1924MB	rvalue checking
time: 2.293; rss: 2628MB	MIR dump
  time: 0.297; rss: 2649MB	SimplifyCfg
  time: 0.474; rss: 2650MB	QualifyAndPromoteConstants
  time: 0.601; rss: 2650MB	TypeckMir
  time: 0.039; rss: 2650MB	SimplifyBranches
  time: 0.136; rss: 2650MB	SimplifyCfg
time: 1.547; rss: 2650MB	MIR cleanup and validation
time: 1.984; rss: 2658MB	borrow checking
time: 0.023; rss: 2659MB	reachability checking
time: 0.328; rss: 2662MB	death checking
time: 0.000; rss: 2662MB	unused lib feature checking
time: 2.008; rss: 2662MB	lint checking
time: 0.000; rss: 2662MB	resolving dependency formats
  time: 0.033; rss: 2662MB	NoLandingPads
  time: 0.133; rss: 2662MB	SimplifyCfg
  time: 0.373; rss: 2678MB	EraseRegions
  time: 0.069; rss: 2678MB	AddCallGuards
  time: 0.991; rss: 2713MB	ElaborateDrops
  time: 0.033; rss: 2713MB	NoLandingPads
  time: 0.282; rss: 2714MB	SimplifyCfg
  time: 0.185; rss: 2714MB	InstCombine
  time: 0.082; rss: 2714MB	Deaggregator
  time: 0.038; rss: 2714MB	CopyPropagation
  time: 0.241; rss: 2715MB	SimplifyLocals
  time: 0.092; rss: 2717MB	AddCallGuards
  time: 0.033; rss: 2717MB	PreTrans
time: 2.586; rss: 2717MB	MIR optimisations
  time: 0.683; rss: 2819MB	write metadata
  time: 5.005; rss: 2895MB	translation item collection
  time: 1.267; rss: 2939MB	codegen unit partitioning
  time: 0.674; rss: 5032MB	internalize symbols
time: 47.199; rss: 5033MB	translation
time: 0.001; rss: 5033MB	assert dep graph
fatal runtime error: out of memory
error: Could not compile `script`.
