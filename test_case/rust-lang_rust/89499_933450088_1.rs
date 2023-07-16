
$ llvm-profdata overlap before-rustc-pgo.profdata after-rustc-pgo.profdata
Profile overlap infomation for base_profile: before-rustc-pgo.profdata and test_profile: after-rustc-pgo.profdata
Program level:
  # of functions overlap: 260504
  # of functions only in test_profile: 89905
  Edge profile overlap: 40.433%
  Percentage of Edge profile only in test_profile: 59.566%
  Edge profile base count sum: 230860056427
  Edge profile test count sum: 571155717471
  IndirectCall profile overlap: 18.746%
  Percentage of IndirectCall profile only in test_profile: 81.254%
  IndirectCall profile base count sum: 722242054
  IndirectCall profile test count sum: 3853524645
  MemOP profile overlap: 21.263%
  Percentage of MemOP profile only in test_profile: 78.737%
  MemOP profile base count sum: 488641949
  MemOP profile test count sum: 2298031353

$ llvm-profdata overlap before-llvm-pgo.profdata after-llvm-pgo.profdata
Profile overlap infomation for base_profile: before-llvm-pgo.profdata and test_profile: after-llvm-pgo.profdata
Program level:
  # of functions overlap: 84177
  # of functions mismatch: 6928
  # of functions only in test_profile: 2696
  Edge profile overlap: 46.840%
  Mismatched count percentage (Edge): 52.323%
  Percentage of Edge profile only in test_profile: 0.685%
  Edge profile base count sum: 1047797180915
  Edge profile test count sum: 1153062758787
  IndirectCall profile overlap: 40.414%
  Mismatched count percentage (IndirectCall): 59.122%
  Percentage of IndirectCall profile only in test_profile: 0.310%
  IndirectCall profile base count sum: 8901901446
  IndirectCall profile test count sum: 8877805375
  MemOP profile overlap: 84.344%
  Mismatched count percentage (MemOP): 15.542%
  Percentage of MemOP profile only in test_profile: 0.001%
  MemOP profile base count sum: 5916707088
  MemOP profile test count sum: 5921229195
