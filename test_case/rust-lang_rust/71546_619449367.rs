
~/r/i71546> cargo +1.24.0 build
   Compiling i71546 v0.1.0 (file:///Users/aminarria/rust-cleanup/i71546)
error: internal compiler error: broken MIR in NodeId(4) (""): errors selecting obligation: [FulfillmentError(Obligation(predicate=Binder(OutlivesPredicate(<&'a V as std::iter::IntoIterator>::Item, ReStatic)),depth=0),Unimplemented)]
 --> src/lib.rs:7:49
  |
7 |     let csv_str: String = value.into_iter().map(|elem| elem.to_string()).collect::<String>();
  |                                                 ^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

error: Could not compile `i71546`.
