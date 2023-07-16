
DEBUG rustc_symbol_mangling symbol_name(def_id=DefId(0:30 ~ repro[1a7c]::{impl#2}::foo), substs=[Const { ty: usize, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:36 ~ repro[1a7c]::{impl#1}::Foo::{constant#0}), const_param_did: Some(DefId(0:27 ~ repro[1a7c]::SomeFoo::A)) }, substs: [], promoted: None }) }])
DEBUG rustc_symbol_mangling::legacy get_symbol_hash(def_id=DefId(0:30 ~ repro[1a7c]::{impl#2}::foo), parameters=[Const { ty: usize, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:36 ~ repro[1a7c]::{impl#1}::Foo::{constant#0}), const_param_did: Some(DefId(0:27 ~ repro[1a7c]::SomeFoo::A)) }, substs: [], promoted: None }) }])
DEBUG rustc_symbol_mangling::legacy hashing substs [Const { ty: usize, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:36 ~ repro[1a7c]::{impl#1}::Foo::{constant#0}), const_param_did: Some(DefId(0:27 ~ repro[1a7c]::SomeFoo::A)) }, substs: [], promoted: None }) }]
DEBUG rustc_symbol_mangling::legacy hash: 46dc62e32e9b3ee8
...
DEBUG rustc_symbol_mangling symbol_name(def_id=DefId(0:30 ~ repro[1a7c]::{impl#2}::foo), substs=[Const { ty: usize, val: Value(Scalar(0x0000000000000004)) }])
DEBUG rustc_symbol_mangling::legacy get_symbol_hash(def_id=DefId(0:30 ~ repro[1a7c]::{impl#2}::foo), parameters=[Const { ty: usize, val: Value(Scalar(0x0000000000000004)) }])
DEBUG rustc_symbol_mangling::legacy hashing substs [Const { ty: usize, val: Value(Scalar(0x0000000000000004)) }]
DEBUG rustc_symbol_mangling::legacy hash: a3e1ed653bfd1112
