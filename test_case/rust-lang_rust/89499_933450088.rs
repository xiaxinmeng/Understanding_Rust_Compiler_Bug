text
$ llvm-profdata overlap before-rustc-pgo.profdata after-rustc-pgo.profdata
Profile overlap infomation for base_profile: before-rustc-pgo.profdata and test_profile: after-rustc-pgo.profdata
Program level:
  # of functions overlap: 260502
  Edge profile overlap: 99.910%
  Edge profile base count sum: 230867115195
  Edge profile test count sum: 231072749498
  IndirectCall profile overlap: 99.996%
  IndirectCall profile base count sum: 722318628
  IndirectCall profile test count sum: 722335980
  MemOP profile overlap: 99.981%
  MemOP profile base count sum: 488656567
  MemOP profile test count sum: 488639611

$ llvm-profdata overlap before-llvm-pgo.profdata after-llvm-pgo.profdata
Profile overlap infomation for base_profile: before-llvm-pgo.profdata and test_profile: after-llvm-pgo.profdata
Program level:
  # of functions overlap: 96161
  Edge profile overlap: 99.742%
  Edge profile base count sum: 1054888702705
  Edge profile test count sum: 1050720657781
  IndirectCall profile overlap: 99.894%
  IndirectCall profile base count sum: 8906460397
  IndirectCall profile test count sum: 8906009877
  MemOP profile overlap: 99.921%
  MemOP profile base count sum: 5927918788
  MemOP profile test count sum: 5920677320
