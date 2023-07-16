console
$ env RUSTFLAGS=-Ztime-passes cargo t --no-run > /dev/null
$ touch src/lib.rs
$ env RUSTFLAGS=-Ztime-passes cargo t --no-run
   Compiling distributary v0.1.0 (file:///home/jon/dev/projects/distributary)
time: 0.053; rss: 63MB	parsing
time: 0.000; rss: 63MB	recursion limit
time: 0.000; rss: 63MB	crate injection
time: 0.052; rss: 63MB	parsing
time: 0.000; rss: 63MB	recursion limit
time: 0.000; rss: 63MB	crate injection
time: 0.001; rss: 66MB	plugin loading
time: 0.000; rss: 66MB	plugin registration
time: 0.001; rss: 66MB	plugin loading
time: 0.000; rss: 66MB	plugin registration
time: 0.245; rss: 214MB	expansion
time: 0.000; rss: 214MB	maybe building test harness
time: 0.003; rss: 214MB	maybe creating a macro crate
time: 0.000; rss: 214MB	checking for inline asm in case the target doesn't support it
time: 0.008; rss: 214MB	early lint checks
time: 0.003; rss: 214MB	AST validation
time: 0.039; rss: 222MB	name resolution
time: 0.008; rss: 222MB	complete gated feature checking
time: 0.025; rss: 235MB	lowering ast -> hir
time: 0.008; rss: 237MB	indexing hir
time: 0.003; rss: 237MB	attribute checking
time: 0.001; rss: 232MB	language item collection
time: 0.006; rss: 232MB	lifetime resolution
time: 0.000; rss: 232MB	looking for entry point
time: 0.000; rss: 232MB	looking for plugin registrar
time: 0.015; rss: 242MB	region resolution
time: 0.003; rss: 242MB	loop checking
time: 0.001; rss: 242MB	static item recursion checking
time: 0.389; rss: 231MB	expansion
time: 0.017; rss: 232MB	maybe building test harness
time: 0.003; rss: 232MB	maybe creating a macro crate
time: 0.000; rss: 232MB	checking for inline asm in case the target doesn't support it
time: 0.013; rss: 232MB	early lint checks
time: 0.003; rss: 232MB	AST validation
time: 0.089; rss: 243MB	compute_incremental_hashes_map
time: 0.069; rss: 241MB	name resolution
time: 0.010; rss: 241MB	complete gated feature checking
time: 0.043; rss: 256MB	lowering ast -> hir
time: 0.020; rss: 257MB	indexing hir
time: 0.005; rss: 257MB	attribute checking
time: 0.003; rss: 246MB	language item collection
time: 0.009; rss: 246MB	lifetime resolution
time: 0.000; rss: 246MB	looking for entry point
time: 0.000; rss: 246MB	looking for plugin registrar
time: 0.023; rss: 254MB	region resolution
time: 0.007; rss: 254MB	loop checking
time: 0.001; rss: 254MB	static item recursion checking
time: 0.262; rss: 258MB	load_dep_graph
time: 0.005; rss: 249MB	stability index
time: 0.012; rss: 250MB	stability checking
time: 0.034; rss: 262MB	type collecting
time: 0.003; rss: 262MB	variance inference
time: 0.004; rss: 262MB	impl wf inference
time: 0.151; rss: 255MB	compute_incremental_hashes_map
time: 0.068; rss: 281MB	coherence checking
time: 0.071; rss: 283MB	wf checking
time: 0.056; rss: 284MB	item-types checking
time: 0.302; rss: 262MB	load_dep_graph
time: 0.006; rss: 262MB	stability index
time: 0.016; rss: 264MB	stability checking
time: 0.048; rss: 276MB	type collecting
time: 0.007; rss: 276MB	variance inference
time: 0.007; rss: 278MB	impl wf inference
time: 0.072; rss: 297MB	coherence checking
time: 0.085; rss: 299MB	wf checking
time: 0.117; rss: 312MB	item-types checking
time: 1.647; rss: 321MB	item-bodies checking
time: 0.066; rss: 323MB	const checking
time: 0.027; rss: 323MB	privacy checking
time: 0.010; rss: 323MB	intrinsic checking
time: 0.005; rss: 323MB	effect checking
time: 0.018; rss: 323MB	match checking
time: 0.013; rss: 325MB	liveness checking
time: 0.049; rss: 325MB	rvalue checking
time: 0.128; rss: 367MB	MIR dump
  time: 0.019; rss: 367MB	SimplifyCfg
  time: 0.029; rss: 367MB	QualifyAndPromoteConstants
  time: 0.040; rss: 367MB	TypeckMir
  time: 0.002; rss: 367MB	SimplifyBranches
  time: 0.010; rss: 367MB	SimplifyCfg
time: 0.100; rss: 367MB	MIR cleanup and validation
time: 0.159; rss: 369MB	borrow checking
time: 0.003; rss: 369MB	reachability checking
time: 0.014; rss: 369MB	death checking
time: 0.000; rss: 369MB	unused lib feature checking
time: 0.096; rss: 369MB	lint checking
time: 0.000; rss: 369MB	resolving dependency formats
  time: 0.003; rss: 369MB	NoLandingPads
  time: 0.008; rss: 369MB	SimplifyCfg
  time: 0.022; rss: 369MB	EraseRegions
  time: 0.004; rss: 369MB	AddCallGuards
  time: 0.083; rss: 375MB	ElaborateDrops
  time: 0.002; rss: 375MB	NoLandingPads
  time: 0.018; rss: 375MB	SimplifyCfg
  time: 0.000; rss: 375MB	Inline
  time: 0.012; rss: 375MB	InstCombine
  time: 0.005; rss: 375MB	Deaggregator
  time: 0.002; rss: 375MB	CopyPropagation
  time: 0.013; rss: 375MB	SimplifyLocals
  time: 0.003; rss: 375MB	AddCallGuards
  time: 0.001; rss: 375MB	PreTrans
time: 0.178; rss: 375MB	MIR optimisations
  time: 0.025; rss: 380MB	write metadata
time: 2.632; rss: 362MB	item-bodies checking
time: 0.105; rss: 368MB	const checking
time: 0.052; rss: 368MB	privacy checking
time: 0.016; rss: 369MB	intrinsic checking
time: 0.008; rss: 369MB	effect checking
time: 0.026; rss: 369MB	match checking
time: 0.020; rss: 371MB	liveness checking
time: 0.068; rss: 371MB	rvalue checking
time: 0.196; rss: 443MB	MIR dump
  time: 0.026; rss: 443MB	SimplifyCfg
  time: 0.042; rss: 443MB	QualifyAndPromoteConstants
  time: 0.067; rss: 443MB	TypeckMir
  time: 0.004; rss: 443MB	SimplifyBranches
  time: 0.015; rss: 443MB	SimplifyCfg
time: 0.154; rss: 443MB	MIR cleanup and validation
time: 0.266; rss: 443MB	borrow checking
time: 0.002; rss: 443MB	reachability checking
time: 0.015; rss: 443MB	death checking
time: 0.000; rss: 443MB	unused lib feature checking
time: 0.139; rss: 443MB	lint checking
time: 0.013; rss: 443MB	resolving dependency formats
  time: 0.003; rss: 443MB	NoLandingPads
  time: 0.015; rss: 443MB	SimplifyCfg
  time: 0.036; rss: 447MB	EraseRegions
  time: 0.006; rss: 447MB	AddCallGuards
  time: 0.123; rss: 455MB	ElaborateDrops
  time: 0.003; rss: 455MB	NoLandingPads
  time: 0.024; rss: 455MB	SimplifyCfg
  time: 0.000; rss: 455MB	Inline
  time: 0.016; rss: 455MB	InstCombine
  time: 0.007; rss: 455MB	Deaggregator
  time: 0.004; rss: 455MB	CopyPropagation
  time: 0.018; rss: 455MB	SimplifyLocals
  time: 0.005; rss: 455MB	AddCallGuards
  time: 0.002; rss: 455MB	PreTrans
time: 0.261; rss: 455MB	MIR optimisations
  time: 0.000; rss: 455MB	write metadata
  time: 2.681; rss: 438MB	translation item collection
  time: 0.637; rss: 480MB	codegen unit partitioning
  time: 2.946; rss: 529MB	translation item collection
  time: 0.286; rss: 499MB	internalize symbols
time: 5.346; rss: 499MB	translation
time: 0.000; rss: 499MB	assert dep graph
time: 0.477; rss: 489MB	serialize dep graph
  time: 0.002; rss: 395MB	codegen passes [3]
time: 0.083; rss: 395MB	LLVM passes
time: 0.000; rss: 395MB	serialize work products
  time: 0.685; rss: 554MB	codegen unit partitioning
time: 1.331; rss: 399MB	linking
time: 0.007; rss: 55MB	parsing
time: 0.000; rss: 55MB	recursion limit
time: 0.000; rss: 55MB	crate injection
time: 0.000; rss: 55MB	plugin loading
time: 0.000; rss: 55MB	plugin registration
time: 0.151; rss: 196MB	expansion
time: 0.004; rss: 198MB	maybe building test harness
time: 0.000; rss: 198MB	maybe creating a macro crate
time: 0.000; rss: 198MB	checking for inline asm in case the target doesn't support it
time: 0.003; rss: 198MB	early lint checks
time: 0.001; rss: 198MB	AST validation
time: 0.034; rss: 204MB	name resolution
time: 0.001; rss: 204MB	complete gated feature checking
time: 0.006; rss: 204MB	lowering ast -> hir
time: 0.002; rss: 206MB	indexing hir
time: 0.000; rss: 206MB	attribute checking
time: 0.000; rss: 210MB	language item collection
time: 0.001; rss: 210MB	lifetime resolution
time: 0.000; rss: 210MB	looking for entry point
time: 0.000; rss: 210MB	looking for plugin registrar
time: 0.002; rss: 210MB	region resolution
time: 0.000; rss: 210MB	loop checking
time: 0.000; rss: 210MB	static item recursion checking
time: 0.015; rss: 210MB	compute_incremental_hashes_map
time: 0.054; rss: 218MB	load_dep_graph
time: 0.000; rss: 218MB	stability index
time: 0.003; rss: 218MB	stability checking
time: 0.003; rss: 218MB	type collecting
time: 0.000; rss: 218MB	variance inference
time: 0.000; rss: 218MB	impl wf inference
time: 0.000; rss: 218MB	coherence checking
time: 0.002; rss: 218MB	wf checking
time: 0.013; rss: 220MB	item-types checking
  time: 0.337; rss: 557MB	internalize symbols
time: 5.774; rss: 557MB	translation
time: 0.000; rss: 557MB	assert dep graph
time: 0.414; rss: 268MB	item-bodies checking
time: 0.011; rss: 268MB	const checking
time: 0.003; rss: 268MB	privacy checking
time: 0.002; rss: 268MB	intrinsic checking
time: 0.001; rss: 268MB	effect checking
time: 0.003; rss: 268MB	match checking
time: 0.002; rss: 268MB	liveness checking
time: 0.006; rss: 268MB	rvalue checking
time: 0.028; rss: 276MB	MIR dump
  time: 0.003; rss: 276MB	SimplifyCfg
  time: 0.006; rss: 276MB	QualifyAndPromoteConstants
  time: 0.008; rss: 276MB	TypeckMir
  time: 0.000; rss: 276MB	SimplifyBranches
  time: 0.002; rss: 276MB	SimplifyCfg
time: 0.020; rss: 276MB	MIR cleanup and validation
time: 0.032; rss: 276MB	borrow checking
time: 0.000; rss: 276MB	reachability checking
time: 0.003; rss: 276MB	death checking
time: 0.000; rss: 276MB	unused lib feature checking
time: 0.022; rss: 276MB	lint checking
time: 0.014; rss: 276MB	resolving dependency formats
  time: 0.000; rss: 276MB	NoLandingPads
  time: 0.002; rss: 276MB	SimplifyCfg
  time: 0.004; rss: 278MB	EraseRegions
  time: 0.001; rss: 278MB	AddCallGuards
  time: 0.020; rss: 280MB	ElaborateDrops
  time: 0.000; rss: 280MB	NoLandingPads
  time: 0.004; rss: 280MB	SimplifyCfg
  time: 0.000; rss: 280MB	Inline
  time: 0.003; rss: 280MB	InstCombine
  time: 0.000; rss: 280MB	Deaggregator
  time: 0.000; rss: 280MB	CopyPropagation
  time: 0.002; rss: 280MB	SimplifyLocals
  time: 0.001; rss: 280MB	AddCallGuards
  time: 0.000; rss: 280MB	PreTrans
time: 0.038; rss: 280MB	MIR optimisations
  time: 0.000; rss: 280MB	write metadata
time: 0.668; rss: 542MB	serialize dep graph
  time: 0.003; rss: 406MB	codegen passes [9]
time: 0.081; rss: 401MB	LLVM passes
time: 0.000; rss: 401MB	serialize work products
  time: 0.465; rss: 300MB	translation item collection
  time: 0.100; rss: 304MB	codegen unit partitioning
  time: 0.046; rss: 320MB	internalize symbols
time: 1.056; rss: 320MB	translation
time: 0.000; rss: 320MB	assert dep graph
time: 0.110; rss: 324MB	serialize dep graph
  time: 0.002; rss: 307MB	codegen passes [1]
time: 0.021; rss: 308MB	LLVM passes
time: 0.000; rss: 308MB	serialize work products
  time: 3.697; rss: 401MB	running linker
time: 3.711; rss: 401MB	linking
  time: 3.605; rss: 308MB	running linker
time: 3.620; rss: 308MB	linking
    Finished dev [unoptimized + debuginfo] target(s) in 16.75 secs
