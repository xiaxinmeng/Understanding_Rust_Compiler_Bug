
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1315 ~ encoding_rs[5e1f]::utf_16::{impl#0}::decode_to_utf16_raw), const_param_did: None }) (before pass SimplifyCfg-final) at bb218[0]:
                                encountered overlapping memory in `Call` terminator: _454 = bswap::<u16>(move _454) -> bb163
  --> src/utf_16.rs:85:21
   |
85 |                     dest.copy_utf16_from::<LittleEndian>(&mut source)
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:784:26
