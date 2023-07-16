`
[0m[0m[1m[32m   Compiling[0m packed_simd v0.3.1
  time: 0.054; rss: 57MB	parsing
  time: 0.000; rss: 58MB	attributes injection
  time: 0.000; rss: 58MB	recursion limit
  time: 0.000; rss: 58MB	crate injection
  time: 0.000; rss: 58MB	plugin loading
  time: 0.000; rss: 58MB	plugin registration
  time: 0.005; rss: 58MB	pre ast expansion lint checks
    time: 2.550; rss: 369MB	expand crate
    time: 0.000; rss: 369MB	check unused macros
  time: 2.550; rss: 369MB	expansion
  time: 0.000; rss: 369MB	maybe building test harness
  time: 0.012; rss: 369MB	maybe creating a macro crate
  time: 0.048; rss: 370MB	creating allocators
  time: 0.036; rss: 370MB	AST validation
  time: 0.497; rss: 412MB	name resolution
  time: 0.075; rss: 412MB	complete gated feature checking
  time: 0.321; rss: 481MB	lowering ast -> hir
  time: 0.081; rss: 482MB	early lint checks
    time: 0.052; rss: 504MB	validate hir map
  time: 0.353; rss: 504MB	indexing hir
  time: 0.000; rss: 504MB	load query result cache
  time: 0.000; rss: 504MB	looking for entry point
  time: 0.000; rss: 504MB	dep graph tcx init
  time: 0.001; rss: 504MB	looking for plugin registrar
  time: 0.001; rss: 504MB	looking for derive registrar
  time: 0.019; rss: 504MB	loop checking
  time: 0.024; rss: 504MB	attribute checking
    time: 0.000; rss: 515MB	solve_nll_region_constraints(DefId(0/1:2171 ~ packed_simd[a932]::v64[0]::f32x2[0]::{{constant}}[0]))
*snip*
    time: 0.000; rss: 527MB	solve_nll_region_constraints(DefId(0/1:4611 ~ packed_simd[a932]::vSize[0]::{{impl}}[587]::from[0]::U[0]::array[0]::{{constant}}[0]))
  time: 0.636; rss: 527MB	stability checking
  time: 0.124; rss: 527MB	type collecting
  time: 0.003; rss: 527MB	outlives testing
  time: 0.019; rss: 527MB	impl wf inference
    time: 0.000; rss: 1113MB	solve_nll_region_constraints(DefId(0/1:224 ~ packed_simd[a932]::codegen[0]::shuffle[0]::{{impl}}[0]::{{constant}}[0]))
*snip*
    time: 0.000; rss: 1246MB	solve_nll_region_constraints(DefId(0/1:4867 ~ packed_simd[a932]::vPtr[0]::{{impl}}[104]::{{constant}}[0]))
  time: 9.972; rss: 1408MB	coherence checking
  time: 0.002; rss: 1408MB	variance testing
    time: 0.000; rss: 1605MB	solve_nll_region_constraints(DefId(0/1:366 ~ packed_simd[a932]::codegen[0]::v16[0]::{{impl}}[0]::NT[0]::{{constant}}[0]))
*snip*
    time: 0.000; rss: 2013MB	solve_nll_region_constraints(DefId(0/0:4027 ~ packed_simd[a932]::codegen[0]::reductions[0]::mask[0]::{{impl}}[7]::any[0]))
    time: 0.000; rss: 2013MB	solve_nll_region_constraints(DefId(0/0:4053 ~ packed_simd[a932]::codegen[0]::reductions[0]::mask[0]::{{impl}}[17]::any[0]))
  time: 5.040; rss: 2013MB	MIR borrow checking
  time: 0.000; rss: 2013MB	dumping chalk-like clauses
  time: 0.005; rss: 2013MB	MIR effect checking
  time: 0.072; rss: 2018MB	death checking
  time: 0.021; rss: 2018MB	unused lib feature checking
  time: 0.176; rss: 2019MB	lint checking
  time: 0.000; rss: 2019MB	resolving dependency formats
    time: 0.890; rss: 2055MB	write metadata
      time: 0.010; rss: 2055MB	collecting roots
      time: 0.186; rss: 2056MB	collecting mono items
    time: 0.196; rss: 2056MB	monomorphization collection
    time: 0.001; rss: 2056MB	codegen unit partitioning
    time: 0.122; rss: 2060MB	codegen to LLVM IR
    time: 0.000; rss: 2060MB	assert dep graph
    time: 0.000; rss: 2060MB	serialize dep graph
  time: 1.215; rss: 2060MB	codegen
    time: 0.056; rss: 2063MB	llvm function passes [packed_simd.smey8184-cgu.0]
    time: 0.777; rss: 2071MB	llvm module passes [packed_simd.smey8184-cgu.0]
    time: 0.798; rss: 2079MB	codegen passes [packed_simd.smey8184-cgu.0]
  time: 1.703; rss: 1539MB	LLVM passes
  time: 0.000; rss: 1540MB	serialize work products
  time: 0.017; rss: 1540MB	linking
