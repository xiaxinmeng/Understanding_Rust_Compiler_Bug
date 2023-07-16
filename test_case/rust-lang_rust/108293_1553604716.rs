plain
    |
118 |         &self,
    |         ^^^^^
    |         |
    |         types differ in mutability
    |         help: change the self-receiver type to match the trait: `self: &mut MaybeStorageDead`
note: type in trait
   --> compiler/rustc_mir_dataflow/src/framework/mod.rs:300:9
    |
300 |         &mut self,
300 |         &mut self,
    |         ^^^^^^^^^
    = note: expected signature `fn(&mut MaybeStorageDead, &mut impl GenKill<Self::Idx>, &rustc_middle::mir::Statement<'tcx>, rustc_middle::mir::Location)`
               found signature `fn(&MaybeStorageDead, &mut impl GenKill<Self::Idx>, &rustc_middle::mir::Statement<'tcx>, rustc_middle::mir::Location)`
error[E0053]: method `terminator_effect` has an incompatible type for trait
   --> compiler/rustc_mir_dataflow/src/impls/storage_liveness.rs:131:9
    |
131 |         &self,
131 |         &self,
    |         ^^^^^
    |         |
    |         types differ in mutability
    |         help: change the self-receiver type to match the trait: `self: &mut MaybeStorageDead`
note: type in trait
   --> compiler/rustc_mir_dataflow/src/framework/mod.rs:317:9
    |
317 |         &mut self,
317 |         &mut self,
    |         ^^^^^^^^^
    = note: expected signature `fn(&mut MaybeStorageDead, &mut impl GenKill<Self::Idx>, &rustc_middle::mir::Terminator<'tcx>, rustc_middle::mir::Location)`
               found signature `fn(&MaybeStorageDead, &mut impl GenKill<Self::Idx>, &rustc_middle::mir::Terminator<'tcx>, rustc_middle::mir::Location)`
error[E0053]: method `call_return_effect` has an incompatible type for trait
   --> compiler/rustc_mir_dataflow/src/impls/storage_liveness.rs:140:9
    |
140 |         &self,
140 |         &self,
    |         ^^^^^
    |         |
    |         types differ in mutability
    |         help: change the self-receiver type to match the trait: `self: &mut MaybeStorageDead`
note: type in trait
   --> compiler/rustc_mir_dataflow/src/framework/mod.rs:336:9
    |
336 |         &mut self,
336 |         &mut self,
    |         ^^^^^^^^^
    = note: expected signature `fn(&mut MaybeStorageDead, &mut impl GenKill<Self::Idx>, rustc_middle::mir::BasicBlock, CallReturnPlaces<'_, '_>)`
               found signature `fn(&MaybeStorageDead, &mut impl GenKill<Self::Idx>, rustc_middle::mir::BasicBlock, CallReturnPlaces<'_, '_>)`
error[E0053]: method `call_return_effect` has an incompatible type for trait
   --> compiler/rustc_mir_dataflow/src/impls/storage_liveness.rs:140:9
    |
140 |         &self,
140 |         &self,
    |         ^^^^^
    |         |
    |         types differ in mutability
    |         help: change the self-receiver type to match the trait: `self: &mut MaybeStorageDead`
note: type in trait
   --> compiler/rustc_mir_dataflow/src/framework/mod.rs:336:9
    |
336 |         &mut self,
336 |         &mut self,
    |         ^^^^^^^^^
    = note: expected signature `fn(&mut MaybeStorageDead, &mut impl GenKill<Self::Idx>, rustc_middle::mir::BasicBlock, framework::CallReturnPlaces<'_, '_>)`
               found signature `fn(&MaybeStorageDead, &mut impl GenKill<Self::Idx>, rustc_middle::mir::BasicBlock, framework::CallReturnPlaces<'_, '_>)`
error[E0308]: mismatched types
error[E0308]: mismatched types
  --> compiler/rustc_mir_dataflow/src/framework/graphviz.rs:52:27
   |
52 |         Formatter { body, results, style, reachable }
   |                           ^^^^^^^ expected `RefCell<&mut Results<'_, A>>`, found `&mut Results<'_, A>`
   |
   = note:         expected struct `RefCell<&mut Results<'_, A>>`
           found mutable reference `&'res mut Results<'tcx, A>`
help: call `Into::into` on this expression to convert `&'res mut Results<'tcx, A>` into `RefCell<&mut Results<'_, A>>`
   |
52 |         Formatter { body, results.into(), style, reachable }

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> compiler/rustc_mir_dataflow/src/framework/graphviz.rs:52:27
   |
52 |         Formatter { body, results, style, reachable }
   |                           ^^^^^^^ expected `RefCell<&mut Results<'_, A>>`, found `&mut Results<'_, A>`
   |
   = note:         expected struct `RefCell<&mut engine::Results<'_, A>>`
           found mutable reference `&'res mut engine::Results<'tcx, A>`
help: call `Into::into` on this expression to convert `&'res mut engine::Results<'tcx, A>` into `RefCell<&mut engine::Results<'_, A>>`
   |
52 |         Formatter { body, results.into(), style, reachable }

Some errors have detailed explanations: E0053, E0308.
For more information about an error, try `rustc --explain E0053`.
error: could not compile `rustc_mir_dataflow` (lib) due to 4 previous errors
