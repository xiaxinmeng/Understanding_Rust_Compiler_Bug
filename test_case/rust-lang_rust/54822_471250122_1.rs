
error[E0277]: the trait bound `F: Foo` is not satisfied                                                                                                       |    tcx.infer_ctxt().enter(|infcx| {
  --> ./test.rs:14:5                                                                                                                                          |        // let param_env = ty::ParamEnv::empty();
   |                                                                                                                                                          |        let param_env = tcx.param_env(impl_c.def_id);
14 |     const T: T = T::DEFAULT;                                                                                                                             |
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `F`                                                                                  |        let inh = Inherited::new(infcx, impl_c.def_id);
   |                                                                                                                                                          |        let infcx = &inh.infcx;
   = help: consider adding a `where F: Foo` bound                                                                                                             |
                                                                                                                                                              |        // The below is for the most part highly similar to the procedure
error: aborting due to previous error                                                                                                                         |        // for methods above. It is simpler in many respects, especially
                                                                                                                                                              |        // because we shouldn't really have to deal with lifetimes or
For more information about this error, try `rustc --explain E0277`.                                                                                           |        // predicates. In fact some of this should probably be put into
