
D:\Christopher\Documents\Code\Rust\rust-unic>rustup update
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: syncing channel updates for 'beta-x86_64-pc-windows-msvc'
info: syncing channel updates for 'nightly-x86_64-pc-windows-msvc'
info: checking for self-updates

   stable-x86_64-pc-windows-msvc unchanged - rustc 1.24.0 (4d90ac38c 2018-02-12)
     beta-x86_64-pc-windows-msvc unchanged - rustc 1.24.0-beta.12 (ed2c0f084 2018-02-12)
  nightly-x86_64-pc-windows-msvc unchanged - rustc 1.25.0-nightly (27a046e93 2018-02-18)

D:\Christopher\Documents\Code\Rust\rust-unic>cargo +nightly rustc --package=unic-ucd-name -- -Z time-passes
   Compiling unic-ucd-core v0.6.0 (file:///D:/Christopher/Documents/Code/Rust/rust-unic/unic/ucd/core)
   Compiling unic-char-range v0.6.0 (file:///D:/Christopher/Documents/Code/Rust/rust-unic/unic/char/range)
   Compiling unic-utils v0.6.0 (file:///D:/Christopher/Documents/Code/Rust/rust-unic/unic/utils)
   Compiling unic-ucd-name v0.6.0 (file:///D:/Christopher/Documents/Code/Rust/rust-unic/unic/ucd/name)
  time: 0.001; rss: 18MB	parsing
  time: 0.000; rss: 18MB	garbage collect incremental cache directory
  time: 0.000; rss: 18MB	recursion limit
  time: 0.000; rss: 18MB	crate injection
  time: 0.000; rss: 18MB	plugin loading
  time: 0.000; rss: 18MB	plugin registration
  time: 0.000; rss: 19MB	background load prev dep-graph
  time: 0.631; rss: 106MB	expansion
  time: 0.000; rss: 106MB	maybe building test harness
  time: 0.012; rss: 106MB	maybe creating a macro crate
  time: 0.033; rss: 106MB	creating allocators
  time: 0.011; rss: 106MB	AST validation
  time: 0.133; rss: 124MB	name resolution
  time: 0.037; rss: 124MB	complete gated feature checking
  time: 0.000; rss: 124MB	blocked while dep-graph loading finishes
  time: 0.160; rss: 171MB	lowering ast -> hir
  time: 0.063; rss: 171MB	early lint checks
  time: 0.447; rss: 179MB	indexing hir
  time: 0.000; rss: 123MB	load query result cache
  time: 0.000; rss: 123MB	looking for entry point
  time: 0.000; rss: 123MB	looking for plugin registrar
  time: 0.033; rss: 123MB	loop checking
  time: 0.068; rss: 123MB	static item recursion checking
  time: 0.054; rss: 135MB	attribute checking
  time: 0.083; rss: 144MB	stability checking
  time: 0.339; rss: 202MB	type collecting
  time: 0.002; rss: 202MB	outlives testing
  time: 0.001; rss: 202MB	impl wf inference
  time: 0.049; rss: 221MB	coherence checking
  time: 0.001; rss: 221MB	variance testing
  time: 0.282; rss: 262MB	wf checking
  time: 4.531; rss: 337MB	item-types checking
  time: 0.032; rss: 342MB	item-bodies checking
  time: 2.627; rss: 553MB	const checking
  time: 0.174; rss: 554MB	privacy checking
  time: 0.023; rss: 554MB	intrinsic checking
  time: 0.045; rss: 555MB	match checking
  time: 0.010; rss: 555MB	liveness checking
  time: 0.546; rss: 566MB	borrow checking
  time: 0.025; rss: 568MB	MIR borrow checking
  time: 0.008; rss: 568MB	MIR effect checking
  time: 0.044; rss: 569MB	death checking
  time: 0.000; rss: 569MB	unused lib feature checking
  time: 0.163; rss: 569MB	lint checking
  time: 0.000; rss: 569MB	resolving dependency formats
    time: 1.076; rss: 618MB	write metadata
    time: 0.104; rss: 620MB	translation item collection
    time: 0.002; rss: 621MB	codegen unit partitioning
    time: 0.001; rss: 636MB	llvm function passes [49a7n47po4ttqjl7]
    time: 0.000; rss: 637MB	llvm module passes [49a7n47po4ttqjl7]
    time: 0.000; rss: 638MB	llvm function passes [3ayaeypdcro9d6yk]
    time: 0.000; rss: 638MB	llvm module passes [3ayaeypdcro9d6yk]
    time: 0.000; rss: 638MB	llvm function passes [3kfx4ynvkmi2y9i5]
    time: 0.000; rss: 638MB	llvm module passes [3kfx4ynvkmi2y9i5]
    time: 0.000; rss: 638MB	llvm function passes [45nf4z58qqykpcpi]
    time: 0.000; rss: 638MB	llvm module passes [45nf4z58qqykpcpi]
    time: 0.000; rss: 639MB	llvm function passes [kt25z0521ngsjub]
    time: 0.000; rss: 639MB	llvm module passes [kt25z0521ngsjub]
    time: 0.000; rss: 642MB	llvm function passes [2ny9ynlpevlhfa8x]
    time: 0.000; rss: 642MB	llvm module passes [2ny9ynlpevlhfa8x]
    time: 0.000; rss: 645MB	llvm function passes [1im38lueib99jsk0]
    time: 0.000; rss: 645MB	llvm module passes [1im38lueib99jsk0]
    time: 0.019; rss: 662MB	codegen passes [3kfx4ynvkmi2y9i5]
    time: 0.016; rss: 663MB	codegen passes [45nf4z58qqykpcpi]
    time: 0.000; rss: 663MB	llvm function passes [2lyh15q6cjwzy18c]
    time: 0.000; rss: 663MB	llvm module passes [2lyh15q6cjwzy18c]
    time: 0.020; rss: 663MB	codegen passes [kt25z0521ngsjub]
    time: 0.025; rss: 663MB	codegen passes [3ayaeypdcro9d6yk]
    time: 0.016; rss: 663MB	codegen passes [2ny9ynlpevlhfa8x]
    time: 0.013; rss: 663MB	codegen passes [1im38lueib99jsk0]
    time: 0.032; rss: 663MB	codegen passes [49a7n47po4ttqjl7]
    time: 0.007; rss: 663MB	codegen passes [2lyh15q6cjwzy18c]
    time: 0.000; rss: 675MB	llvm function passes [4ypvbwho0bu5tnww]
    time: 0.000; rss: 675MB	llvm module passes [4ypvbwho0bu5tnww]
    time: 0.000; rss: 675MB	llvm function passes [43v6g0y2xsxoggnt]
    time: 0.000; rss: 675MB	llvm module passes [43v6g0y2xsxoggnt]
    time: 0.000; rss: 675MB	llvm function passes [9elsx31vb4it187]
    time: 0.000; rss: 675MB	llvm module passes [9elsx31vb4it187]
    time: 0.000; rss: 676MB	llvm function passes [16u6js6g0l3k1ic6]
    time: 0.000; rss: 676MB	llvm module passes [16u6js6g0l3k1ic6]
    time: 0.000; rss: 676MB	llvm function passes [3e8c0xfx7ikmlnfk]
    time: 0.000; rss: 676MB	llvm module passes [3e8c0xfx7ikmlnfk]
    time: 0.000; rss: 676MB	llvm function passes [9fcb3syd3ne5k0n]
    time: 0.000; rss: 676MB	llvm module passes [9fcb3syd3ne5k0n]
    time: 0.000; rss: 679MB	llvm function passes [c6lbtaiefvx3wya]
    time: 0.000; rss: 679MB	llvm module passes [c6lbtaiefvx3wya]
    time: 0.019; rss: 679MB	codegen passes [43v6g0y2xsxoggnt]
    time: 0.008; rss: 679MB	codegen passes [9fcb3syd3ne5k0n]
    time: 0.016; rss: 679MB	codegen passes [16u6js6g0l3k1ic6]
    time: 0.013; rss: 679MB	codegen passes [3e8c0xfx7ikmlnfk]
    time: 0.000; rss: 679MB	llvm function passes [98g0d9x8aw3akpe]
    time: 0.000; rss: 679MB	llvm module passes [98g0d9x8aw3akpe]
    time: 0.000; rss: 679MB	llvm function passes [2jqywn86b2gsqohu]
    time: 0.000; rss: 679MB	llvm module passes [2jqywn86b2gsqohu]
    time: 0.000; rss: 679MB	llvm function passes [8xzrsc1ux72v29j]
    time: 0.000; rss: 680MB	llvm module passes [8xzrsc1ux72v29j]
    time: 0.016; rss: 680MB	codegen passes [9elsx31vb4it187]
    time: 0.023; rss: 680MB	codegen passes [4ypvbwho0bu5tnww]
    time: 0.000; rss: 680MB	llvm function passes [4ezmh1vbs95c5ack]
    time: 0.000; rss: 680MB	llvm function passes [2kjrmm4fe2aha78f]
    time: 0.000; rss: 680MB	llvm module passes [2kjrmm4fe2aha78f]
    time: 0.000; rss: 680MB	llvm module passes [4ezmh1vbs95c5ack]
    time: 0.000; rss: 680MB	llvm function passes [4yh8x2b62dcih00t]
    time: 0.000; rss: 680MB	llvm module passes [4yh8x2b62dcih00t]
    time: 0.011; rss: 681MB	codegen passes [98g0d9x8aw3akpe]
    time: 0.010; rss: 681MB	codegen passes [2jqywn86b2gsqohu]
    time: 0.000; rss: 681MB	llvm function passes [1mvmz58owquyropc]
    time: 0.000; rss: 681MB	llvm module passes [1mvmz58owquyropc]
    time: 0.016; rss: 681MB	codegen passes [c6lbtaiefvx3wya]
    time: 0.000; rss: 681MB	llvm function passes [4xq48u46a1pwiqn7]
    time: 0.000; rss: 681MB	llvm module passes [4xq48u46a1pwiqn7]
    time: 0.016; rss: 681MB	codegen passes [4ezmh1vbs95c5ack]
    time: 0.000; rss: 681MB	llvm function passes [2xnvmuhjbhd7vxcm]
    time: 0.000; rss: 681MB	llvm module passes [2xnvmuhjbhd7vxcm]
    time: 0.000; rss: 681MB	llvm function passes [2f0hry2t7c05ttdi]
    time: 0.000; rss: 681MB	llvm module passes [2f0hry2t7c05ttdi]
    time: 0.020; rss: 681MB	codegen passes [2kjrmm4fe2aha78f]
    time: 0.020; rss: 681MB	codegen passes [8xzrsc1ux72v29j]
    time: 0.008; rss: 681MB	codegen passes [1mvmz58owquyropc]
    time: 0.000; rss: 681MB	llvm function passes [4jdnq7xfjeka1bt]
    time: 0.000; rss: 681MB	llvm module passes [4jdnq7xfjeka1bt]
    time: 0.000; rss: 681MB	llvm function passes [48721dc4k5qxei0u]
    time: 0.000; rss: 681MB	llvm module passes [48721dc4k5qxei0u]
    time: 0.014; rss: 681MB	codegen passes [4yh8x2b62dcih00t]
    time: 0.000; rss: 681MB	llvm function passes [v6ozwtpojmqfurc]
    time: 0.000; rss: 681MB	llvm module passes [v6ozwtpojmqfurc]
    time: 0.000; rss: 682MB	llvm function passes [1y16o1qfye96o7m0]
    time: 0.000; rss: 682MB	llvm module passes [1y16o1qfye96o7m0]
    time: 0.009; rss: 682MB	codegen passes [4xq48u46a1pwiqn7]
    time: 0.690; rss: 682MB	translate to LLVM IR
    time: 0.000; rss: 682MB	assert dep graph
    time: 0.000; rss: 682MB	llvm function passes [1dqvxks6k2bzkxe]
    time: 0.000; rss: 682MB	llvm module passes [1dqvxks6k2bzkxe]
    time: 0.009; rss: 682MB	codegen passes [2f0hry2t7c05ttdi]
    time: 0.011; rss: 682MB	codegen passes [2xnvmuhjbhd7vxcm]
    time: 0.000; rss: 682MB	llvm function passes [23tqyymcb18u96mb]
    time: 0.000; rss: 682MB	llvm function passes [524bze3gcv99ucga]
    time: 0.000; rss: 682MB	llvm module passes [23tqyymcb18u96mb]
    time: 0.000; rss: 682MB	llvm module passes [524bze3gcv99ucga]
    time: 0.010; rss: 682MB	codegen passes [4jdnq7xfjeka1bt]
    time: 0.000; rss: 682MB	llvm function passes [2r82puffnvvb8iic]
    time: 0.000; rss: 682MB	llvm module passes [2r82puffnvvb8iic]
    time: 0.014; rss: 682MB	codegen passes [v6ozwtpojmqfurc]
    time: 0.010; rss: 682MB	codegen passes [48721dc4k5qxei0u]
    time: 0.009; rss: 682MB	codegen passes [1dqvxks6k2bzkxe]
    time: 0.014; rss: 682MB	codegen passes [1y16o1qfye96o7m0]
    time: 0.008; rss: 687MB	codegen passes [524bze3gcv99ucga]
    time: 0.008; rss: 687MB	codegen passes [23tqyymcb18u96mb]
    time: 0.006; rss: 685MB	codegen passes [2r82puffnvvb8iic]
    time: 0.111; rss: 690MB	llvm function passes [2iv7jmandrgcbb7e]
    time: 0.000; rss: 690MB	llvm module passes [2iv7jmandrgcbb7e]
      time: 0.727; rss: 692MB	persist query result cache
      time: 0.133; rss: 733MB	persist dep-graph
    time: 0.860; rss: 733MB	serialize dep graph
  time: 2.863; rss: 733MB	translation
    time: 1.419; rss: 276MB	codegen passes [2iv7jmandrgcbb7e]
  time: 2.886; rss: 237MB	LLVM passes
  time: 0.002; rss: 237MB	serialize work products
  time: 0.905; rss: 238MB	linking
    Finished dev [unoptimized + debuginfo] target(s) in 20.55 secs
