
src/librustc/mir/visit.rs:348:30: 348:42 error: this function takes 3 parameters but 2 parameters were supplied [E0061]

src/librustc/mir/visit.rs:348                         self.visit_lvalue(lvalue, LvalueContext::Store);

                                                           ^~~~~~~~~~~~

src/librustc/mir/visit.rs:767:1: 767:29 note: in this expansion of make_mir_visitor! (defined in src/librustc/mir/visit.rs)

src/librustc/mir/visit.rs:348:30: 348:42 help: run `rustc --explain E0061` to see a detailed explanation

src/librustc/mir/visit.rs:348:30: 348:42 note: the following parameter types were expected: &mir::repr::Lvalue<'tcx>, mir::visit::LvalueContext, mir::repr::Location

src/librustc/mir/visit.rs:348:30: 348:42 error: this function takes 3 parameters but 2 parameters were supplied [E0061]

src/librustc/mir/visit.rs:348                         self.visit_lvalue(lvalue, LvalueContext::Store);

                                                           ^~~~~~~~~~~~

src/librustc/mir/visit.rs:768:1: 768:35 note: in this expansion of make_mir_visitor! (defined in src/librustc/mir/visit.rs)

src/librustc/mir/visit.rs:348:30: 348:42 help: run `rustc --explain E0061` to see a detailed explanation

src/librustc/mir/visit.rs:348:30: 348:42 note: the following parameter types were expected: &mut mir::repr::Lvalue<'tcx>, mir::visit::LvalueContext, mir::repr::Location

cp: x86_64-unknown-linux-gnu/stage1/lib/libsyntax_ext
