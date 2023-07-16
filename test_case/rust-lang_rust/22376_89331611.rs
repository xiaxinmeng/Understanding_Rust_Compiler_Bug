
/home/aidan/rust/aidancully/rust/src/librustc/util/ppaux.rs:685:50: 685:66 error: the trait `core::fmt::Display` is not implemented for the type `syntax::ast::DefId` [E0277]
/home/aidan/rust/aidancully/rust/src/librustc/util/ppaux.rs:685             format!("local_trait_not_ready({})", trait_ref.def_id)
                                                                                                                 ^~~~~~~~~~~~~~~~
note: in expansion of format_args!
<std macros>:2:26: 2:59 note: expansion site
<std macros>:1:1: 2:61 note: in expansion of format!
/home/aidan/rust/aidancully/rust/src/librustc/util/ppaux.rs:685:13: 686:10 note: expansion site
/home/aidan/rust/aidancully/rust/src/librustc/util/ppaux.rs:685:50: 685:66 note: `syntax::ast::DefId` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
/home/aidan/rust/aidancully/rust/src/librustc/util/ppaux.rs:685             format!("local_trait_not_ready({})", trait_ref.def_id)
                                                                                                                 ^~~~~~~~~~~~~~~~
note: in expansion of format_args!
<std macros>:2:26: 2:59 note: expansion site
<std macros>:1:1: 2:61 note: in expansion of format!
/home/aidan/rust/aidancully/rust/src/librustc/util/ppaux.rs:685:13: 686:10 note: expansion site
error: aborting due to previous error
/home/aidan/rust/aidancully/rust/mk/target.mk:167: recipe for target 'x86_64-unknown-freebsd/stage0/lib/rustlib/x86_64-unknown-freebsd/lib/stamp.rustc' failed
gmake: *** [x86_64-unknown-freebsd/stage0/lib/rustlib/x86_64-unknown-freebsd/lib/stamp.rustc] Error 101
