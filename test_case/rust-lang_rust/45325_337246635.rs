
[01:06:16] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[01:06:16] 	thread '[mir-opt] mir-opt/validate_1.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:06:16] Expected Line: "        Validate(Suspend(ReScope(Remainder(BlockRemainder { block: ItemLocalId(22), first_statement_index: middle::region::FirstStatementIndex0 }))), [(*_3): i32]); // scope 2 at /checkout/src/test/mir-opt/validate_1.rs:26:37: 26:40"
[01:06:16] Actual Line: "        Validate(Suspend(ReScope(Remainder(BlockRemainder { block: ItemLocalId(22), first_statement_index: 0 }))), [(*_3): i32]);"
[01:06:16] Expected:
[01:06:16] ... (elided)
[01:06:16] fn main::{{closure}}(_1: &ReErased [closure@NodeId(50)], _2: &ReErased mut i32) -> i32 {
[01:06:16] ... (elided)
[01:06:16]     bb0: {
[01:06:16]         Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), index: DefIndex(1:11) => validate_1[8cd8]::main[0]::{{closure}}[0] }, BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId { krate: CrateNum(0), index: DefIndex(1:11) => validate_1[8cd8]::main[0]::{{closure}}[0] }, BrAnon(1)) mut i32]);
[01:06:16]         StorageLive(_3);
[01:06:16]         _3 = _2;
[01:06:16]         StorageLive(_4);
[01:06:16]         Validate(Suspend(ReScope(Remainder(BlockRemainder { block: ItemLocalId(22), first_statement_index: 0 }))), [(*_3): i32]);
[01:06:16]         _4 = &ReErased (*_3);
[01:06:16]         Validate(Acquire, [(*_4): i32/ReScope(Remainder(BlockRemainder { block: ItemLocalId(22), first_statement_index: 0 })) (imm)]);
[01:06:16]         StorageLive(_5);
[01:06:16]         _5 = (*_4);
[01:06:16]         _0 = _5;
[01:06:16]         StorageDead(_5);
[01:06:16]         EndRegion(ReScope(Remainder(BlockRemainder { block: ItemLocalId(22), first_statement_index: 0 })));
[01:06:16]         StorageDead(_4);
[01:06:16]         StorageDead(_3);
[01:06:16]         return;
[01:06:16]     }
[01:06:16] }
[01:06:16] Actual:
[01:06:16] fn main::{{closure}}(_1: &ReErased [closure@NodeId(50)], _2: &ReErased mut i32) -> i32 {
[01:06:16]     let mut _0: i32;
[01:06:16]     scope 1 {
[01:06:16]         let _3: &ReErased mut i32;
[01:06:16]         scope 2 {
[01:06:16]             let _4: &ReErased i32;
[01:06:16]         }
[01:06:16]         scope 3 {
[01:06:16]         }
[01:06:16]     }
[01:06:16]     let mut _5: i32;
[01:06:16]     bb0: {                              
[01:06:16]         Validate(Acquire, [_1: &ReFree(DefId { krate: CrateNum(0), index: DefIndex(1:11) => validate_1[8cd8]::main[0]::{{closure}}[0] }, BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId { krate: CrateNum(0), index: DefIndex(1:11) => validate_1[8cd8]::main[0]::{{closure}}[0] }, BrAnon(1)) mut i32]);
[01:06:16]         StorageLive(_3);
[01:06:16]         _3 = _2;
[01:06:16]         StorageLive(_4);
[01:06:16]         Validate(Suspend(ReScope(Remainder(BlockRemainder { block: ItemLocalId(22), first_statement_index: middle::region::FirstStatementIndex0 }))), [(*_3): i32]);
[01:06:16]         _4 = &ReErased (*_3);
[01:06:16]         Validate(Acquire, [(*_4): i32/ReScope(Remainder(BlockRemainder { block: ItemLocalId(22), first_statement_index: middle::region::FirstStatementIndex0 })) (imm)]);
[01:06:16]         StorageLive(_5);
[01:06:16]         _5 = (*_4);
[01:06:16]         _0 = _5;
[01:06:16]         StorageDead(_5);
[01:06:16]         EndRegion(ReScope(Remainder(BlockRemainder { block: ItemLocalId(22), first_statement_index: middle::region::FirstStatementIndex0 })));
[01:06:16]         StorageDead(_4);
[01:06:16]         StorageDead(_3);
[01:06:16]         return;
[01:06:16]     }
[01:06:16] }', /checkout/src/tools/compiletest/src/runtest.rs:2324:12
[01:06:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:16] 
[01:06:16] ---- [mir-opt] mir-opt/validate_3.rs stdout ----
[01:06:16] 	thread '[mir-opt] mir-opt/validate_3.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:06:16] Expected Line: "        Validate(Suspend(ReScope(Remainder(BlockRemainder { block: ItemLocalId(19), first_statement_index: middle::region::FirstStatementIndex3 }))), [_1: Test]); // scope 3 at /checkout/src/test/mir-opt/validate_3.rs:26:13: 26:15"
[01:06:16] Actual Line: "        Validate(Suspend(ReScope(Remainder(BlockRemainder { block: ItemLocalId(19), first_statement_index: 3 }))), [_1: Test]);"
[01:06:16] Expected:
[01:06:16] ... (elided)
[01:06:16] fn main() -> () {
[01:06:16] ... (elided)
[01:06:16]     let mut _5: &ReErased i32;
[01:06:16]     bb0: {
[01:06:16]         StorageLive(_1);
[01:06:16]         _1 = Test { x: const 0i32 };
[01:06:16]         StorageLive(_2);
[01:06:16]         Validate(Suspend(ReScope(Remainder(BlockRemainder { block: ItemLocalId(19), first_statement_index: 3 }))), [_1: Test]);
[01:06:16]         _2 = &ReErased _1;
[01:06:16]         Validate(Acquire, [(*_2): Test/ReScope(Remainder(BlockRemainder { block: ItemLocalId(19), first_statement_index: 3 })) (imm)]);
[01:06:16]         StorageLive(_4);
[01:06:16]         StorageLive(_5);
[01:06:16]         Validate(Suspend(ReScope(Node(ItemLocalId(17)))), [((*_2).0: i32): i32/ReScope(Remainder(BlockRemainder { block: ItemLocalId(19), first_statement_index: 3 })) (imm)]);
[01:06:16]         _5 = &ReErased ((*_2).0: i32);
[01:06:16]         Validate(Acquire, [(*_5): i32/ReScope(Node(ItemLocalId(17))) (imm)]);
[01:06:16]         Validate(Suspend(ReScope(Node(ItemLocalId(17)))), [(*_5): i32/ReScope(Node(ItemLocalId(17))) (imm)]);
[01:06:16]         _4 = &ReErased (*_5);
[01:06:16]         Validate(Acquire, [(*_4): i32/ReScope(Node(ItemLocalId(17))) (imm)]);
[01:06:16]         Validate(Release, [_3: (), _4: &ReScope(Node(ItemLocalId(17))) i32]);
[01:06:16]         _3 = const foo(_4) -> bb1;
[01:06:16]     }
[01:06:16]     bb1: {
[01:06:16]         Validate(Acquire, [_3: ()]);
[01:06:16]         EndRegion(ReScope(Node(ItemLocalId(17))));
[01:06:16]         StorageDead(_4);
[01:06:16]         StorageDead(_5);
[01:06:16]         _0 = ();
[01:06:16]         EndRegion(ReScope(Remainder(BlockRemainder { block: ItemLocalId(19), first_statement_index: 3 })));
[01:06:16]         StorageDead(_2);
[01:06:16]         StorageDead(_1);
[01:06:16]         return;
[01:06:16]     }
[01:06:16] }
[01:06:16] Actual:
[01:06:16] fn main() -> () {
[01:06:16]     let mut _0: ();
[01:06:16]     scope 1 {
[01:06:16]         let _1: Test;
[01:06:16]         scope 3 {
[01:06:16]             let _2: &ReErased Test;
[01:06:16]         }
[01:06:16]         scope 4 {
[01:06:16]         }
[01:06:16]     }
[01:06:16]     scope 2 {
[01:06:16]     }
[01:06:16]     let mut _3: ();
[01:06:16]     let mut _4: &ReErased i32;
[01:06:16]     let mut _5: &ReErased i32;
[01:06:16]     bb0: {                              
[01:06:16]         StorageLive(_1);
[01:06:16]         _1 = Test { x: const 0i32 };
[01:06:16]         StorageLive(_2);
[01:06:16]         Validate(Suspend(ReScope(Remainder(BlockRemainder { block: ItemLocalId(19), first_statement_index: middle::region::FirstStatementIndex3 }))), [_1: Test]);
[01:06:16]         _2 = &ReErased _1;
[01:06:16]         Validate(Acquire, [(*_2): Test/ReScope(Remainder(BlockRemainder { block: ItemLocalId(19), first_statement_index: middle::region::FirstStatementIndex3 })) (imm)]);
[01:06:16]         StorageLive(_4);
[01:06:16]         StorageLive(_5);
[01:06:16]         Validate(Suspend(ReScope(Node(ItemLocalId(17)))), [((*_2).0: i32): i32/ReScope(Remainder(BlockRemainder { block: ItemLocalId(19), first_statement_index: middle::region::FirstStatementIndex3 })) (imm)]);
[01:06:16]         _5 = &ReErased ((*_2).0: i32);
[01:06:16]         Validate(Acquire, [(*_5): i32/ReScope(Node(ItemLocalId(17))) (imm)]);
[01:06:16]         Validate(Suspend(ReScope(Node(ItemLocalId(17)))), [(*_5): i32/ReScope(Node(ItemLocalId(17))) (imm)]);
[01:06:16]         _4 = &ReErased (*_5);
[01:06:16]         Validate(Acquire, [(*_4): i32/ReScope(Node(ItemLocalId(17))) (imm)]);
[01:06:16]         Validate(Release, [_3: (), _4: &ReScope(Node(ItemLocalId(17))) i32]);
[01:06:16]         _3 = const foo(_4) -> bb1;
[01:06:16]     }
[01:06:16]     bb1: {                              
[01:06:16]         Validate(Acquire, [_3: ()]);
[01:06:16]         EndRegion(ReScope(Node(ItemLocalId(17))));
[01:06:16]         StorageDead(_4);
[01:06:16]         StorageDead(_5);
[01:06:16]         _0 = ();
[01:06:16]         EndRegion(ReScope(Remainder(BlockRemainder { block: ItemLocalId(19), first_statement_index: middle::region::FirstStatementIndex3 })));
[01:06:16]         StorageDead(_2);
[01:06:16]         StorageDead(_1);
[01:06:16]         return;
[01:06:16]     }
[01:06:16] }', /checkout/src/tools/compiletest/src/runtest.rs:2324:12
[01:06:16] 
[01:06:16] 
[01:06:16] failures:
[01:06:16]     [mir-opt] mir-opt/validate_1.rs
[01:06:16]     [mir-opt] mir-opt/validate_3.rs
[01:06:16] 
[01:06:16] test result: FAILED. 30 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:16] 
[01:06:16] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:329:21
