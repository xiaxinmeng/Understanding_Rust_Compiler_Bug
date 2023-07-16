plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0308]: mismatched types
  --> compiler/rustc_traits/src/chalk/lowering.rs:57:43
   |
57 |         chalk_ir::Substitution::from_iter(interner, self.iter().map(|s| s.lower_into(interner)))
   |                                           ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
   |
   |
57 |         chalk_ir::Substitution::from_iter(*interner, self.iter().map(|s| s.lower_into(interner)))

error[E0308]: mismatched types
  --> compiler/rustc_traits/src/chalk/lowering.rs:63:42
   |
   |
63 |         interner.tcx.mk_substs(self.iter(interner).map(|subst| subst.lower_into(interner)))
   |                                          ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
   |
   |
63 |         interner.tcx.mk_substs(self.iter(*interner).map(|subst| subst.lower_into(interner)))

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:118:52
    |
    |
118 |                 conditions: chalk_ir::Goals::empty(interner),
    |                                                    ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
118 |                 conditions: chalk_ir::Goals::empty(*interner),

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:120:59
    |
    |
120 |                 constraints: chalk_ir::Constraints::empty(interner),
    |                                                           ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
120 |                 constraints: chalk_ir::Constraints::empty(*interner),

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:122:88
    |
    |
122 |             chalk_ir::ProgramClauseData(chalk_ir::Binders::new(binders, value)).intern(interner)
    |                                                                                        ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
122 |             chalk_ir::ProgramClauseData(chalk_ir::Binders::new(binders, value)).intern(*interner)

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:128:62
    |
    |
128 |                 clauses: chalk_ir::ProgramClauses::from_iter(&interner, clauses),
    |                                                              ^^^^^^^^^ expected struct `ChalkRustInterner`, found `&&ChalkRustInterner<'tcx>`
error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:130:31
    |
    |
130 |             goal: goal.intern(&interner),
    |                               ^^^^^^^^^ expected struct `ChalkRustInterner`, found `&&ChalkRustInterner<'tcx>`

error[E0277]: the trait bound `&ChalkRustInterner<'tcx>: Interner` is not satisfied
    --> compiler/rustc_traits/src/chalk/lowering.rs:173:72
     |
173  |                         chalk_ir::GoalData::All(chalk_ir::Goals::empty(interner))
     |                                                 ---------------------- ^^^^^^^^ the trait `Interner` is not implemented for `&ChalkRustInterner<'tcx>`
     |                                                 required by a bound introduced by this call
     |
     = help: the following implementations were found:
     = help: the following implementations were found:
               <ChalkRustInterner<'tcx> as Interner>
note: required by a bound in `Goals::<I>::empty`
     |
2885 | / macro_rules! interned_slice_common {
2885 | / macro_rules! interned_slice_common {
2886 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2887 | |         /// List of interned elements.
2888 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
...    |
2893 | |         impl<I: Interner> $seq<I> {
     | |                 ^^^^^^^^ required by this bound in `Goals::<I>::empty`
2929 | |     };
2930 | | }
2930 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2932 | / macro_rules! interned_slice {
2932 | / macro_rules! interned_slice {
2933 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2934 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2935 | |
...    |
2966 | |     };
2967 | | }
2967 | | }
     | |_- in this expansion of `interned_slice!` (#1)
...
2993 |   interned_slice!(Goals, goals_data => Goal<I>, intern_goals => InternedGoals);


error[E0277]: the trait bound `&ChalkRustInterner<'tcx>: Interner` is not satisfied
    --> compiler/rustc_traits/src/chalk/lowering.rs:173:49
     |
173  |                         chalk_ir::GoalData::All(chalk_ir::Goals::empty(interner))
     |                         ----------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Interner` is not implemented for `&ChalkRustInterner<'tcx>`
     |                         required by a bound introduced by this call
     |
     = help: the following implementations were found:
     = help: the following implementations were found:
               <ChalkRustInterner<'tcx> as Interner>
note: required by a bound in `Goals`
     |
2885 | / macro_rules! interned_slice_common {
2885 | / macro_rules! interned_slice_common {
2886 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2887 | |         /// List of interned elements.
2888 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2889 | |         pub struct $seq<I: Interner> {
     | |                            ^^^^^^^^ required by this bound in `Goals`
2929 | |     };
2930 | | }
2930 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2932 | / macro_rules! interned_slice {
2932 | / macro_rules! interned_slice {
2933 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2934 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2935 | |
...    |
2966 | |     };
2967 | | }
2967 | | }
     | |_- in this expansion of `interned_slice!` (#1)
...
2993 |   interned_slice!(Goals, goals_data => Goal<I>, intern_goals => InternedGoals);


error[E0277]: the trait bound `&ChalkRustInterner<'tcx>: Interner` is not satisfied
    --> compiler/rustc_traits/src/chalk/lowering.rs:173:25
     |
173  |                         chalk_ir::GoalData::All(chalk_ir::Goals::empty(interner))
     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Interner` is not implemented for `&ChalkRustInterner<'tcx>`
     = help: the following implementations were found:
     = help: the following implementations were found:
               <ChalkRustInterner<'tcx> as Interner>
note: required by a bound in `GoalData`
     |
     |
2582 | pub enum GoalData<I: Interner> {
     |                      ^^^^^^^^ required by this bound in `GoalData`

error[E0308]: `match` arms have incompatible types
   --> compiler/rustc_traits/src/chalk/lowering.rs:176:26
    |
168 |                   GenericArgKind::Type(ty) => match ty.kind() {
    |                                               --------------- `match` arms have incompatible types
...
173 |                           chalk_ir::GoalData::All(chalk_ir::Goals::empty(interner))
    |                           --------------------------------------------------------- this is found to be of type `GoalData<&ChalkRustInterner<'tcx>>`
...
176 |                       _ => chalk_ir::GoalData::DomainGoal(chalk_ir::DomainGoal::WellFormed(
    |  __________________________^
177 | |                         chalk_ir::WellFormed::Ty(ty.lower_into(interner)),
178 | |                     )),
    | |______________________^ expected `&ChalkRustInterner<'tcx>`, found struct `ChalkRustInterner`
    |
    = note: expected type `GoalData<&ChalkRustInterner<'tcx>>`
               found enum `GoalData<ChalkRustInterner<'_>>`

error[E0277]: the trait bound `&ChalkRustInterner<'tcx>: Interner` is not satisfied
    --> compiler/rustc_traits/src/chalk/lowering.rs:182:68
     |
182  |                     chalk_ir::GoalData::All(chalk_ir::Goals::empty(interner))
     |                                             ---------------------- ^^^^^^^^ the trait `Interner` is not implemented for `&ChalkRustInterner<'tcx>`
     |                                             required by a bound introduced by this call
     |
     = help: the following implementations were found:
     = help: the following implementations were found:
               <ChalkRustInterner<'tcx> as Interner>
note: required by a bound in `Goals::<I>::empty`
     |
2885 | / macro_rules! interned_slice_common {
2885 | / macro_rules! interned_slice_common {
2886 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2887 | |         /// List of interned elements.
2888 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
...    |
2893 | |         impl<I: Interner> $seq<I> {
     | |                 ^^^^^^^^ required by this bound in `Goals::<I>::empty`
2929 | |     };
2930 | | }
2930 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2932 | / macro_rules! interned_slice {
2932 | / macro_rules! interned_slice {
2933 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2934 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2935 | |
...    |
2966 | |     };
2967 | | }
2967 | | }
     | |_- in this expansion of `interned_slice!` (#1)
...
2993 |   interned_slice!(Goals, goals_data => Goal<I>, intern_goals => InternedGoals);


error[E0277]: the trait bound `&ChalkRustInterner<'tcx>: Interner` is not satisfied
    --> compiler/rustc_traits/src/chalk/lowering.rs:182:45
     |
182  |                     chalk_ir::GoalData::All(chalk_ir::Goals::empty(interner))
     |                     ----------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Interner` is not implemented for `&ChalkRustInterner<'tcx>`
     |                     required by a bound introduced by this call
     |
     = help: the following implementations were found:
     = help: the following implementations were found:
               <ChalkRustInterner<'tcx> as Interner>
note: required by a bound in `Goals`
     |
2885 | / macro_rules! interned_slice_common {
2885 | / macro_rules! interned_slice_common {
2886 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2887 | |         /// List of interned elements.
2888 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2889 | |         pub struct $seq<I: Interner> {
     | |                            ^^^^^^^^ required by this bound in `Goals`
2929 | |     };
2930 | | }
2930 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2932 | / macro_rules! interned_slice {
2932 | / macro_rules! interned_slice {
2933 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2934 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2935 | |
...    |
2966 | |     };
2967 | | }
2967 | | }
     | |_- in this expansion of `interned_slice!` (#1)
...
2993 |   interned_slice!(Goals, goals_data => Goal<I>, intern_goals => InternedGoals);


error[E0277]: the trait bound `&ChalkRustInterner<'tcx>: Interner` is not satisfied
    --> compiler/rustc_traits/src/chalk/lowering.rs:182:21
     |
182  |                     chalk_ir::GoalData::All(chalk_ir::Goals::empty(interner))
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Interner` is not implemented for `&ChalkRustInterner<'tcx>`
     = help: the following implementations were found:
     = help: the following implementations were found:
               <ChalkRustInterner<'tcx> as Interner>
note: required by a bound in `GoalData`
     |
     |
2582 | pub enum GoalData<I: Interner> {
     |                      ^^^^^^^^ required by this bound in `GoalData`

error[E0277]: the trait bound `&ChalkRustInterner<'tcx>: Interner` is not satisfied
    --> compiler/rustc_traits/src/chalk/lowering.rs:200:64
     |
200  |                 chalk_ir::GoalData::All(chalk_ir::Goals::empty(interner))
     |                                         ---------------------- ^^^^^^^^ the trait `Interner` is not implemented for `&ChalkRustInterner<'tcx>`
     |                                         required by a bound introduced by this call
     |
     = help: the following implementations were found:
     = help: the following implementations were found:
               <ChalkRustInterner<'tcx> as Interner>
note: required by a bound in `Goals::<I>::empty`
     |
2885 | / macro_rules! interned_slice_common {
2885 | / macro_rules! interned_slice_common {
2886 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2887 | |         /// List of interned elements.
2888 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
...    |
2893 | |         impl<I: Interner> $seq<I> {
     | |                 ^^^^^^^^ required by this bound in `Goals::<I>::empty`
2929 | |     };
2930 | | }
2930 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2932 | / macro_rules! interned_slice {
2932 | / macro_rules! interned_slice {
2933 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2934 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2935 | |
...    |
2966 | |     };
2967 | | }
2967 | | }
     | |_- in this expansion of `interned_slice!` (#1)
...
2993 |   interned_slice!(Goals, goals_data => Goal<I>, intern_goals => InternedGoals);


error[E0277]: the trait bound `&ChalkRustInterner<'tcx>: Interner` is not satisfied
    --> compiler/rustc_traits/src/chalk/lowering.rs:200:41
     |
200  |                 chalk_ir::GoalData::All(chalk_ir::Goals::empty(interner))
     |                 ----------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Interner` is not implemented for `&ChalkRustInterner<'tcx>`
     |                 required by a bound introduced by this call
     |
     = help: the following implementations were found:
     = help: the following implementations were found:
               <ChalkRustInterner<'tcx> as Interner>
note: required by a bound in `Goals`
     |
2885 | / macro_rules! interned_slice_common {
2885 | / macro_rules! interned_slice_common {
2886 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2887 | |         /// List of interned elements.
2888 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2889 | |         pub struct $seq<I: Interner> {
     | |                            ^^^^^^^^ required by this bound in `Goals`
2929 | |     };
2930 | | }
2930 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2932 | / macro_rules! interned_slice {
2932 | / macro_rules! interned_slice {
2933 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2934 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2935 | |
...    |
2966 | |     };
2967 | | }
2967 | | }
     | |_- in this expansion of `interned_slice!` (#1)
...
2993 |   interned_slice!(Goals, goals_data => Goal<I>, intern_goals => InternedGoals);


error[E0277]: the trait bound `&ChalkRustInterner<'tcx>: Interner` is not satisfied
    --> compiler/rustc_traits/src/chalk/lowering.rs:200:17
     |
200  |                 chalk_ir::GoalData::All(chalk_ir::Goals::empty(interner))
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Interner` is not implemented for `&ChalkRustInterner<'tcx>`
     = help: the following implementations were found:
     = help: the following implementations were found:
               <ChalkRustInterner<'tcx> as Interner>
note: required by a bound in `GoalData`
     |
     |
2582 | pub enum GoalData<I: Interner> {
     |                      ^^^^^^^^ required by this bound in `GoalData`
error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:290:46
    |
    |
290 |                     num_binders: binders.len(interner),
    |                                              ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
290 |                     num_binders: binders.len(*interner),

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:293:25
    |
    |
293 |                         interner,
    |                         ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
293 |                         *interner,

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:295:90
    |
    |
295 | ...                   chalk_ir::GenericArgData::Ty(ty.lower_into(interner)).intern(interner)
    |                                                                                    ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
295 |                             chalk_ir::GenericArgData::Ty(ty.lower_into(interner)).intern(*interner)

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:334:17
    |
    |
334 |         .intern(interner)
    |                 ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
334 |         .intern(*interner)

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:342:36
    |
    |
342 |         let kind = match self.kind(interner) {
    |                                    ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
342 |         let kind = match self.kind(*interner) {

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:443:21
    |
    |
443 |             .intern(interner),
    |                     ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
443 |             .intern(*interner),

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:445:63
    |
    |
445 |             ReStatic => chalk_ir::LifetimeData::Static.intern(interner),
    |                                                               ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
445 |             ReStatic => chalk_ir::LifetimeData::Static.intern(*interner),

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:452:25
    |
    |
452 |                 .intern(interner)
    |                         ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
452 |                 .intern(*interner)

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:463:36
    |
    |
463 |         let kind = match self.data(interner) {
    |                                    ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
463 |         let kind = match self.data(*interner) {

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:501:50
    |
    |
501 |         chalk_ir::ConstData { ty, value }.intern(interner)
    |                                                  ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
501 |         chalk_ir::ConstData { ty, value }.intern(*interner)

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:507:30
    |
    |
507 |         let data = self.data(interner);
    |                              ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
507 |         let data = self.data(*interner);

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:535:17
    |
    |
535 |         .intern(interner)
    |                 ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
535 |         .intern(*interner)

error[E0308]: mismatched types
   --> compiler/rustc_traits/src/chalk/lowering.rs:543:25
    |
    |
543 |         match self.data(interner) {
    |                         ^^^^^^^^ expected struct `ChalkRustInterner`, found `&ChalkRustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
543 |         match self.data(*interner) {


error[E0277]: the trait bound `&ChalkRustInterner<'tcx>: Interner` is not satisfied
    --> compiler/rustc_traits/src/chalk/lowering.rs:671:34
     |
671  |         let existential_binder = chalk_ir::VariableKinds::from1(
