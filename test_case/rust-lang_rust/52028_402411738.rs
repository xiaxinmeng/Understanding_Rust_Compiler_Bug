
            .      mir: &'a Mir<'tcx>,
            .      regioncx: &Rc<RegionInferenceContext<'tcx>>, 
            .      borrows_out_of_scope_at_location: &mut FxHashMap<Location, Vec<BorrowIndex>>,
            .      borrow_index: BorrowIndex,
            .      borrow_region: RegionVid,
            .      location: Location,
            .  ) {     
            .      // Keep track of places we've locations to check and locations that we have checked.
       46,668      let mut stack = vec![ location ];
            .      let mut visited = FxHashSet();
       62,224      visited.insert(location);
    4,413,266  => /home/njn/moz/rust0/src/libstd/collections/hash/set.rs:<std::collections::hash::set::HashSet<T, S>>::insert (15431x)
            .                  
            .      debug!(     
            .          "borrow {:?} has region {:?} with value {:?}",
            .          borrow_index,
            .          borrow_region,
            .          regioncx.region_value_str(borrow_region),
            .      );      
            .      debug!("borrow {:?} starts at {:?}", borrow_index, location);
  971,769,410      while let Some(location) = stack.pop() {
            .          // If region does not contain a point at the location, then add to list and skip
            .          // successor locations.
  971,769,410          if !regioncx.region_contains_point(borrow_region, location) {
            .              debug!("borrow {:?} gets killed at {:?}", borrow_index, location);
       99,190              borrows_out_of_scope_at_location
    1,290,825  => /home/njn/moz/rust0/src/libstd/collections/hash/map.rs:<std::collections::hash::map::Entry<'a, K, V>>::or_insert (13827x)
    3,298,140  => /home/njn/moz/rust0/src/libstd/collections/hash/map.rs:<std::collections::hash::map::HashMap<K, V, S>>::entry (13827x)
       28,340                  .entry(location)
            .                  .or_insert(vec![])
            .                  .push(borrow_index);
            .              continue;
            .          }
            .
            .          let bb_data = &mir[location.block];
            .          // If this is the last statement in the block, then add the
            .          // terminator successors next.
1,457,611,605          if location.statement_index == bb_data.statements.len() {
            .              // Add successors to locations to visit, if not visited before.
       59,584              if let Some(ref terminator) = bb_data.terminator {
       89,376                  for block in terminator.successors() {
      799,865  => /home/njn/moz/rust0/src/librustc/mir/mod.rs:rustc::mir::Terminator::successors (29527x)
      190,405                      let loc = block.start_location();
      112,488  => /home/njn/moz/rust0/src/librustc/mir/mod.rs:rustc::mir::BasicBlock::start_location (37496x)
      228,486                      if visited.insert(loc) {
    5,583,147  => /home/njn/moz/rust0/src/libstd/collections/hash/set.rs:<std::collections::hash::set::HashSet<T, S>>::insert (37496x)
            .                          stack.push(loc);
            .                      }
            .                  }
            .              }
            .          } else {
            .              // Visit next statement in block.
2,429,203,715              let loc = location.successor_within_block();
1,943,360,660  => /home/njn/moz/rust0/src/librustc/mir/mod.rs:rustc::mir::Location::successor_within_block (485840165x)
2,915,044,458              if visited.insert(loc) {
68,678,149,511  => /home/njn/moz/rust0/src/libstd/collections/hash/set.rs:<std::collections::hash::set::HashSet<T, S>>::insert (485840165x)
            .                  stack.push(loc);
            .              }
            .          }
            .      }
            .  }
