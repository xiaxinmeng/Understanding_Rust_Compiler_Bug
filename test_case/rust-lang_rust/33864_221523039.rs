
../src/librustc_resolve/lib.rs:610:31: 610:48 error: attempted access of field `explicit_self` on type `&syntax::ast::MethodSig`, but no field with that name was found
../src/librustc_resolve/lib.rs:610                 MethodRibKind(sig.explicit_self.node == SelfKind::Static)
                                                                 ^~~~~~~~~~~~~~~~~
../src/librustc_resolve/lib.rs:610:57: 610:73 error: no associated item named `Static` found for type `syntax::ast::SelfKind` in the current scope
../src/librustc_resolve/lib.rs:610                 MethodRibKind(sig.explicit_self.node == SelfKind::Static)
                                                                                           ^~~~~~~~~~~~~~~~
../src/librustc_resolve/lib.rs:1680:62: 1680:79 error: attempted access of field `explicit_self` on type `&syntax::ast::MethodSig`, but no field with that name was found
../src/librustc_resolve/lib.rs:1680                                                              sig.explicit_self.node ==
                                                                                                 ^~~~~~~~~~~~~~~~~
../src/librustc_resolve/lib.rs:1681:62: 1681:78 error: no associated item named `Static` found for type `syntax::ast::SelfKind` in the current scope
../src/librustc_resolve/lib.rs:1681                                                              SelfKind::Static));
                                                                                                 ^~~~~~~~~~~~~~~~
../src/librustc_resolve/lib.rs:2011:61: 2011:78 error: attempted access of field `explicit_self` on type `&syntax::ast::MethodSig`, but no field with that name was found
../src/librustc_resolve/lib.rs:2011                                                             sig.explicit_self.node ==
                                                                                                ^~~~~~~~~~~~~~~~~
../src/librustc_resolve/lib.rs:2012:61: 2012:77 error: no associated item named `Static` found for type `syntax::ast::SelfKind` in the current scope
../src/librustc_resolve/lib.rs:2012                                                             SelfKind::Static));
                                                                                                ^~~~~~~~~~~~~~~~
error: aborting due to 6 previous errors
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_resolve] Error 101
