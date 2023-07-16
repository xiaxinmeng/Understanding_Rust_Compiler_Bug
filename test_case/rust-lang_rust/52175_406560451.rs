plain
[00:06:39] error[E0408]: variable `Return` is not bound in all patterns
[00:06:39]     --> librustc/mir/mod.rs:1042:13
[00:06:39]      |
[00:06:39] 1042 |               Resume
[00:06:39]      |               ^^^^^^ pattern doesn't bind `Return`
[00:06:39] 1043 |               | Abort
[00:06:39]      |                 ^^^^^ pattern doesn't bind `Return`
[00:06:39] 1044 |               | GeneratorDrop
[00:06:39]      |                 ^^^^^^^^^^^^^ pattern doesn't bind `Return`
[00:06:39] 1045 |               | Return
[00:06:39]      |                 ------ variable not in all patterns
[00:06:39] 1046 |               | Unreachable
[00:06:39]      |                 ^^^^^^^^^^^ pattern doesn't bind `Return`
[00:06:39] 1047 |               | Call {
[00:06:39] 1048 | |                 destination: None,
[00:06:39] 1049 | |                 cleanup: None,
[00:06:39] 1050 | |                 ..
[00:06:39] 1050 | |                 ..
[00:06:39] 1051 | |             } => None.into_iter().chain(&[]),
[00:06:39]      | |_____________^ pattern doesn't bind `Return`
[00:06:39] error[E0408]: variable `Return` is not bound in all patterns
[00:06:39]     --> librustc/mir/mod.rs:1127:13
[00:06:39]      |
[00:06:39] 1127 |               Resume
[00:06:39] 1127 |               Resume
[00:06:39]      |               ^^^^^^ pattern doesn't bind `Return`
[00:06:39] 1128 |               | Abort
[00:06:39]      |                 ^^^^^ pattern doesn't bind `Return`
[00:06:39] 1129 |               | GeneratorDrop
[00:06:39]      |                 ^^^^^^^^^^^^^ pattern doesn't bind `Return`
[00:06:39] 1130 |               | Return
[00:06:39]      |                 ------ variable not in all patterns
[00:06:39] 1131 |               | Unreachable
[00:06:39]      |                 ^^^^^^^^^^^ pattern doesn't bind `Return`
[00:06:39] 1132 |               | Call {
[00:06:39] 1133 | |                 destination: None,
[00:06:39] 1134 | |                 cleanup: None,
[00:06:39] 1135 | |                 ..
[00:06:39] 1135 | |                 ..
[00:06:39] 1136 | |             } => None.into_iter().chain(&mut []),
[00:06:39]      | |_____________^ pattern doesn't bind `Return`
[00:06:39] error[E0408]: variable `Return` is not bound in all patterns
[00:06:39]     --> librustc/mir/mod.rs:1448:22
[00:06:39]      |
[00:06:39]      |
[00:06:39] 1448 |             Return | Resume | Abort | Unreachable | GeneratorDrop => vec![],
[00:06:39]      |             ------   ^^^^^^   ^^^^^   ^^^^^^^^^^^   ^^^^^^^^^^^^^ pattern doesn't bind `Return`
[00:06:39]      |             |        |        |       pattern doesn't bind `Return`
[00:06:39]      |             |        |        pattern doesn't bind `Return`
[00:06:39]      |             |        pattern doesn't bind `Return`
[00:06:39]      |             variable not in all patterns
[00:06:39]      |             variable not in all patterns
[00:06:39] 
[00:06:39] error[E0408]: variable `Return` is not bound in all patterns
[00:06:39]     --> librustc/mir/mod.rs:2754:13
[00:06:39]      |
[00:06:39] 2754 |             Goto { .. }
[00:06:39]      |             ^^^^^^^^^^^ pattern doesn't bind `Return`
[00:06:39] 2755 |             | Resume
[00:06:39]      |               ^^^^^^ pattern doesn't bind `Return`
[00:06:39] 2756 |             | Abort
[00:06:39]      |               ^^^^^ pattern doesn't bind `Return`
[00:06:39] 2757 |             | Return
[00:06:39]      |               ------ variable not in all patterns
[00:06:39] 2758 |             | GeneratorDrop
[00:06:39]      |               ^^^^^^^^^^^^^ pattern doesn't bind `Return`
[00:06:39] 2759 |             | Unreachable
[00:06:39]      |               ^^^^^^^^^^^ pattern doesn't bind `Return`
[00:06:39] 2760 |             | FalseEdges { .. }
[00:06:39]      |               ^^^^^^^^^^^^^^^^^ pattern doesn't bind `Return`
[00:06:39] 2761 |             | FalseUnwind { .. } => false,
[00:06:39]      |               ^^^^^^^^^^^^^^^^^^ pattern doesn't bind `Return`
[00:06:39] 
[00:06:51] error[E0599]: no variant named `Return` found for type `mir::TerminatorKind<'_>` in the current scope
[00:06:51]    --> librustc/ich/impls_mir.rs:172:13
[00:06:51]     |
[00:06:51] 172 |             mir::TerminatorKind::Return |
[00:06:51]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ variant not found in `mir::TerminatorKind<'_>`
[00:06:51]    ::: librustc/mir/mod.rs:851:1
[00:06:51]     |
[00:06:51]     |
[00:06:51] 851 | pub enum TerminatorKind<'tcx> {
[00:06:51]     | ----------------------------- variant `Return` not found here
[00:06:51] 
[00:06:56] error[E0599]: no variant named `Return` found for type `mir::TerminatorKind<'_>` in the current scope
[00:06:56]      |
[00:06:56]      |
[00:06:56] 851  | pub enum TerminatorKind<'tcx> {
[00:06:56]      | ----------------------------- variant `Return` not found here
[00:06:56] ...
[00:06:56] 1218 |             | TerminatorKind::Return
[00:06:56]      |               ^^^^^^^^^^^^^^^^^^^^^^ variant not found in `mir::TerminatorKind<'_>`
[00:06:56] 
[00:06:56] error[E0599]: no variant named `Return` found for type `mir::TerminatorKind<'_>` in the current scope
[00:06:56]      |
[00:06:56]      |
[00:06:56] 851  | pub enum TerminatorKind<'tcx> {
[00:06:56]      | ----------------------------- variant `Return` not found here
[00:06:56] ...
[00:06:56] 1243 |             | TerminatorKind::Return
[00:06:56]      |               ^^^^^^^^^^^^^^^^^^^^^^ variant not found in `mir::TerminatorKind<'_>`
[00:06:56] 
[00:06:57] error[E0599]: no variant named `Return` found for type `mir::TerminatorKind<'_>` in the current scope
[00:06:57]    --> librustc/mir/visit.rs:443:21
[00:06:57] 443 |                     TerminatorKind::Return |
[00:06:57]     |                     ^^^^^^^^^^^^^^^^^^^^^^ variant not found in `mir::TerminatorKind<'_>`
[00:06:57] ...
[00:06:57] ...
[00:06:57] 819 | make_mir_visitor!(Visitor,);
[00:06:57]     | ---------------------------- in this macro invocation
[00:06:57]    ::: librustc/mir/mod.rs:851:1
[00:06:57]     |
[00:06:57]     |
[00:06:57] 851 | pub enum TerminatorKind<'tcx> {
[00:06:57]     | ----------------------------- variant `Return` not found here
[00:06:57] 
[00:06:57] error[E0599]: no variant named `Return` found for type `mir::TerminatorKind<'_>` in the current scope
[00:06:57]    --> librustc/mir/visit.rs:443:21
[00:06:57] 443 |                     TerminatorKind::Return |
[00:06:57]     |                     ^^^^^^^^^^^^^^^^^^^^^^ variant not found in `mir::TerminatorKind<'_>`
[00:06:57] ...
[00:06:57] ...
[00:06:57] 820 | make_mir_visitor!(MutVisitor,mut);
[00:06:57]     | ---------------------------------- in this macro invocation
[00:06:57]    ::: librustc/mir/mod.rs:851:1
[00:06:57]     |
[00:06:57]     |
[00:06:57] 851 | pub enum TerminatorKind<'tcx> {
[00:06:57]     | ----------------------------- variant `Return` not found here
Fri, 20 Jul 2018 10:28:45 GMT
travis_time:end:140165cc:start=1532082525180009113,finish=1532082525299056199,duration=119047086

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
