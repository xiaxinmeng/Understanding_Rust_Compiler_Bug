
[DEBUG rustc::dep_graph::cgu_reuse_tracker] set_actual_reuse("37it3t5gwe6y9f2", PostLto)
[INFO  rustc_codegen_llvm::back::lto]  - mks4f5lxe2i4sqc: re-used
...
  = note: lld: error: undefined symbol: _$LT$rubble..ble..Duration$u20$as$u20$core..fmt..Display$GT$::fmt::hcaf51ebe05a39b3b
          >>> referenced by mks4f5lxe2i4sqc
          >>>               /tmp/issue59535/out/rubble2/rubble.mks4f5lxe2i4sqc.rcgu.o:(_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$::fmt::he13cc5d4e65d5ab9)
