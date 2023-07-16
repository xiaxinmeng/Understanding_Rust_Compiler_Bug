
jturner-23759:rust jturner$ git grep E0046
src/doc/book/associated-constants.md:error: not all trait items implemented, missing: `ID` [E0046]
src/librustc_typeck/check/mod.rs:        span_err!(tcx.sess, impl_span, E0046,
src/librustc_typeck/diagnostics.rs:E0046: r##"
src/test/compile-fail/E0046.rs:impl Foo for Bar {} //~ ERROR E0046
src/test/compile-fail/impl-wrong-item-for-trait.rs:    //~^ ERROR E0046
src/test/compile-fail/impl-wrong-item-for-trait.rs:    //~^ ERROR E0046
src/test/compile-fail/impl-wrong-item-for-trait.rs:    //~^ ERROR E0046
src/test/compile-fail/issue-23729.rs:            //~^ ERROR not all trait items implemented, missing: `Item` [E0046]
src/test/compile-fail/issue-23827.rs:    //~^ ERROR not all trait items implemented, missing: `Output` [E0046]
src/test/compile-fail/issue-24356.rs:            //~^ ERROR not all trait items implemented, missing: `Target` [E0046]
