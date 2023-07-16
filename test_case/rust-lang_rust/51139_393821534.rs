
error: internal compiler error: broken MIR in NodeId(3244) (""): errors selecting obligation: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<Q as std::marker::Sized>)),depth=1),Unimplemented)]
  --> C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\linked-hash-map-0.3.0\src\lib.rs:86:1
   |
86 | struct Qey<Q: ?Sized>(Q);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
error: aborting due to previous error
