
rustc_driver_fd22acb2f5822090!ZN12rustc_middle9dep_graph111_$LT$impl$u20$rustc_query_system..dep_graph..DepKind$u20$for$u20$rustc_middle..dep_graph..dep_node..DepKind$GT$9read_deps17haf2565c75f65db83E+0x24:
00007ff9`5f91ea24 e83778f1ff      call    rustc_driver_fd22acb2f5822090!ZN12rustc_middle2ty7context3tls3TLV7__getit17h3f90482d1bd7ab7eE (00007ff9`5f836260)
0:004> k
 # Child-SP          RetAddr               Call Site
00 000000ad`4b606fd0 00007ff9`5f66ba1c     rustc_driver_fd22acb2f5822090!ZN12rustc_middle9dep_graph111_$LT$impl$u20$rustc_query_system..dep_graph..DepKind$u20$for$u20$rustc_middle..dep_graph..dep_node..DepKind$GT$9read_deps17haf2565c75f65db83E+0x24
01 000000ad`4b607090 00007ff9`5f52bf95     rustc_driver_fd22acb2f5822090!ZN18rustc_query_system9dep_graph5graph21DepGraphData$LT$K$GT$10read_index17h80df760fadc7c348E+0x1c
02 000000ad`4b6070d0 00007ff9`5f6949bf     rustc_driver_fd22acb2f5822090!ZN18rustc_query_system5query8plumbing14get_query_impl17h25df09cb289d452bE+0x105
03 000000ad`4b607310 00007ff9`5f0b2350     rustc_driver_fd22acb2f5822090!ZN123_$LT$rustc_middle..ty..layout..LayoutCx$LT$rustc_middle..ty..context..TyCtxt$GT$$u20$as$u20$rustc_target..abi..LayoutOf$GT$9layout_of17h835ba83aa866799aE+0x8f
04 000000ad`4b6074e0 00007ff9`5f0b1f10     rustc_driver_fd22acb2f5822090!ZN10rustc_lint7builtin26ClashingExternDeclarations22structurally_same_type17h74b463e0803287f3E+0x9b0
05 000000ad`4b607590 00007ff9`5f0b1ba7     rustc_driver_fd22acb2f5822090!ZN10rustc_lint7builtin26ClashingExternDeclarations22structurally_same_type17h74b463e0803287f3E+0x570
06 000000ad`4b6076a0 00007ff9`5f08daf4     rustc_driver_fd22acb2f5822090!ZN10rustc_lint7builtin26ClashingExternDeclarations22structurally_same_type17h74b463e0803287f3E+0x207
07 000000ad`4b6077b0 00007ff9`5f0b1fa2     rustc_driver_fd22acb2f5822090!ZN4core4iter6traits8iterator8Iterator5eq_by17h8d3fc6b26777f72fE+0x264
08 000000ad`4b6078e0 00007ff9`5f0b1ba7     rustc_driver_fd22acb2f5822090!ZN10rustc_lint7builtin26ClashingExternDeclarations22structurally_same_type17h74b463e0803287f3E+0x602
09 000000ad`4b6079f0 00007ff9`5f08daf4     rustc_driver_fd22acb2f5822090!ZN10rustc_lint7builtin26ClashingExternDeclarations22structurally_same_type17h74b463e0803287f3E+0x207
0a 000000ad`4b607b00 00007ff9`5f0b1fa2     rustc_driver_fd22acb2f5822090!ZN4core4iter6traits8iterator8Iterator5eq_by17h8d3fc6b26777f72fE+0x264
0b 000000ad`4b607c30 00007ff9`5f0b1ba7     rustc_driver_fd22acb2f5822090!ZN10rustc_lint7builtin26ClashingExternDeclarations22structurally_same_type17h74b463e0803287f3E+0x602
0c 000000ad`4b607d40 00007ff9`5f08daf4     rustc_driver_fd22acb2f5822090!ZN10rustc_lint7builtin26ClashingExternDeclarations22structurally_same_type17h74b463e0803287f3E+0x207
