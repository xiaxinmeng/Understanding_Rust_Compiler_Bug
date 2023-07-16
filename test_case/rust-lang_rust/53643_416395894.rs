
--------------------------------------------------------------------------------
-- Auto-annotated source: /home/njn/moz/rust0/src/librustc_mir/dataflow/impls/borrows.rs
--------------------------------------------------------------------------------
           Ir 

-- line 57 ----------------------------------------
            .      mir: &Mir<'tcx>,
            .      regioncx: &Rc<RegionInferenceContext<'tcx>>,
            .      borrows_out_of_scope_at_location: &mut FxHashMap<Location, Vec<BorrowIndex>>,
            .      borrow_index: BorrowIndex,
            .      borrow_region: RegionVid,
            .      location: Location,
            .  ) {
            .      // Keep track of places we've locations to check and locations that we have checked.
       73,356      let mut stack = vec![ location ];
            .      let mut visited = FxHashSet();
       97,808      visited.insert(location);
    7,114,950  => /home/njn/moz/rust0/src/libstd/collections/hash/set.rs:<std::collections::hash::set::HashSet<T, S>>::insert (24450x)
            .  
            .      debug!(
            .          "borrow {:?} has region {:?} with value {:?}",
            .          borrow_index,
            .          borrow_region,
            .          regioncx.region_value_str(borrow_region),
            .      );
            .      debug!("borrow {:?} starts at {:?}", borrow_index, location);
2,237,113,404      while let Some(location) = stack.pop() {
            .          // If region does not contain a point at the location, then add to list and skip
            .          // successor locations.
5,592,783,510          if !regioncx.region_contains(borrow_region, location) {
52,572,164,242  => /home/njn/moz/rust0/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:rustc_mir::borrow_check::nll::region_infer::RegionInferenceContext::region_contains (1,118,556,686x)
            .              debug!("borrow {:?} gets killed at {:?}", borrow_index, location);
       19,740              borrows_out_of_scope_at_location
      561,036  => /home/njn/moz/rust0/src/libstd/collections/hash/map.rs:<std::collections::hash::map::Entry<'a, K, V>>::or_default (3944x)
      967,818  => /home/njn/moz/rust0/src/libstd/collections/hash/map.rs:<std::collections::hash::map::HashMap<K, V, S>>::entry (3944x)
        7,896                  .entry(location)
            .                  .or_default()
            .                  .push(borrow_index);
            .              continue;
            .          }
            .  
1,118,552,754          let bb_data = &mir[location.block];
            .          // If this is the last statement in the block, then add the
            .          // terminator successors next.
3,355,658,262          if location.statement_index == bb_data.statements.len() {
            .              // Add successors to locations to visit, if not visited before.
       52,110              if let Some(ref terminator) = bb_data.terminator {
       78,165                  for block in terminator.successors() {
      646,293  => /home/njn/moz/rust0/src/librustc/mir/mod.rs:rustc::mir::Terminator::successors (26053x)
       21,546                      let loc = block.start_location();
       21,534  => /home/njn/moz/rust0/src/librustc/mir/mod.rs:rustc::mir::BasicBlock::start_location (7178x)
       35,910                      if visited.insert(loc) {
      985,265  => /home/njn/moz/rust0/src/libstd/collections/hash/set.rs:<std::collections::hash::set::HashSet<T, S>>::insert (7178x)
            .                          stack.push(loc);
            .                      }
            .                  }
            .              }
            .          } else {
            .              // Visit next statement in block.
3,355,580,097              let loc = location.successor_within_block();
4,474,106,756  => /home/njn/moz/rust0/src/librustc/mir/mod.rs:rustc::mir::Location::successor_within_block (1,118,526,689x)
5,592,633,495              if visited.insert(loc) {
156,161,136,069  => /home/njn/moz/rust0/src/libstd/collections/hash/set.rs:<std::collections::hash::set::HashSet<T, S>>::insert (1,118,526,689x)
            .                  stack.push(loc);
            .              }
            .          }
            .      }
            .  }
            .  
