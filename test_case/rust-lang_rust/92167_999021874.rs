plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0053]: method `intern_ty` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:209:18
    |
209 |     fn intern_ty(&self, ty: chalk_ir::TyKind<Self>) -> Self::InternedType {
    |                  |
    |                  |
    |                  expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, chalk_ir::TyKind<_>) -> std::boxed::Box<_>`
               found fn pointer `fn(&RustInterner<'tcx>, chalk_ir::TyKind<_>) -> std::boxed::Box<_>`

error[E0053]: method `ty_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:214:20
    |
214 |     fn ty_data<'a>(&self, ty: &'a Self::InternedType) -> &'a chalk_ir::TyData<Self> {
    |                    |
    |                    |
    |                    expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a std::boxed::Box<TyData<RustInterner<'_>>>) -> &'a TyData<RustInterner<'tcx>>`
               found fn pointer `fn(&RustInterner<'tcx>, &std::boxed::Box<TyData<RustInterner<'_>>>) -> &TyData<RustInterner<'tcx>>`

error[E0053]: method `intern_lifetime` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:218:24
    |
218 |     fn intern_lifetime(&self, lifetime: chalk_ir::LifetimeData<Self>) -> Self::InternedLifetime {
    |                        |
    |                        |
    |                        expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, LifetimeData<_>) -> std::boxed::Box<_>`
               found fn pointer `fn(&RustInterner<'tcx>, LifetimeData<_>) -> std::boxed::Box<_>`

error[E0053]: method `lifetime_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:223:9
223 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a std::boxed::Box<LifetimeData<RustInterner<'_>>>) -> &'a LifetimeData<RustInterner<'tcx>>`
               found fn pointer `fn(&RustInterner<'tcx>, &std::boxed::Box<LifetimeData<RustInterner<'_>>>) -> &LifetimeData<RustInterner<'tcx>>`
error[E0053]: method `intern_const` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:229:21
    |
    |
229 |     fn intern_const(&self, constant: chalk_ir::ConstData<Self>) -> Self::InternedConst {
    |                     |
    |                     |
    |                     expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, ConstData<_>) -> std::boxed::Box<_>`
               found fn pointer `fn(&RustInterner<'tcx>, ConstData<_>) -> std::boxed::Box<_>`
error[E0053]: method `const_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:233:23
    |
    |
233 |     fn const_data<'a>(&self, constant: &'a Self::InternedConst) -> &'a chalk_ir::ConstData<Self> {
    |                       |
    |                       |
    |                       expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a std::boxed::Box<ConstData<RustInterner<'_>>>) -> &'a ConstData<RustInterner<'tcx>>`
               found fn pointer `fn(&RustInterner<'tcx>, &std::boxed::Box<ConstData<RustInterner<'_>>>) -> &ConstData<RustInterner<'tcx>>`
error[E0053]: method `const_eq` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:238:9
    |
238 |         &self,
238 |         &self,
    |         ^^^^^
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &std::boxed::Box<TyData<RustInterner<'_>>>, &value::ConstValue<'_>, &value::ConstValue<'_>) -> _`
               found fn pointer `fn(&RustInterner<'tcx>, &std::boxed::Box<TyData<RustInterner<'_>>>, &value::ConstValue<'_>, &value::ConstValue<'_>) -> _`

error[E0053]: method `intern_generic_arg` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:246:27
    |
246 |     fn intern_generic_arg(&self, data: chalk_ir::GenericArgData<Self>) -> Self::InternedGenericArg {
    |                           |
    |                           |
    |                           expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, GenericArgData<_>) -> std::boxed::Box<_>`
               found fn pointer `fn(&RustInterner<'tcx>, GenericArgData<_>) -> std::boxed::Box<_>`

error[E0053]: method `generic_arg_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:251:9
251 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a std::boxed::Box<GenericArgData<RustInterner<'_>>>) -> &'a GenericArgData<RustInterner<'tcx>>`
               found fn pointer `fn(&RustInterner<'tcx>, &std::boxed::Box<GenericArgData<RustInterner<'_>>>) -> &GenericArgData<RustInterner<'tcx>>`

error[E0053]: method `intern_goal` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:257:20
    |
257 |     fn intern_goal(&self, goal: chalk_ir::GoalData<Self>) -> Self::InternedGoal {
    |                    |
    |                    |
    |                    expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, GoalData<_>) -> std::boxed::Box<_>`
               found fn pointer `fn(&RustInterner<'tcx>, GoalData<_>) -> std::boxed::Box<_>`

error[E0053]: method `goal_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:261:22
    |
261 |     fn goal_data<'a>(&self, goal: &'a Self::InternedGoal) -> &'a chalk_ir::GoalData<Self> {
    |                      |
    |                      |
    |                      expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a std::boxed::Box<GoalData<RustInterner<'_>>>) -> &'a GoalData<RustInterner<'tcx>>`
               found fn pointer `fn(&RustInterner<'tcx>, &std::boxed::Box<GoalData<RustInterner<'_>>>) -> &GoalData<RustInterner<'tcx>>`

error[E0053]: method `intern_goals` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:266:9
266 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, _) -> Result<_, _>`
               found fn pointer `fn(&RustInterner<'tcx>, _) -> Result<_, _>`

error[E0053]: method `goals_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:272:23
    |
272 |     fn goals_data<'a>(&self, goals: &'a Self::InternedGoals) -> &'a [chalk_ir::Goal<Self>] {
    |                       |
    |                       |
    |                       expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a Vec<Goal<RustInterner<'_>>>) -> &'a [Goal<RustInterner<'tcx>>]`
               found fn pointer `fn(&RustInterner<'tcx>, &Vec<Goal<RustInterner<'_>>>) -> &[Goal<RustInterner<'tcx>>]`
error[E0053]: method `intern_substitution` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:277:9
    |
277 |         &self,
277 |         &self,
    |         ^^^^^
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, _) -> Result<_, _>`
               found fn pointer `fn(&RustInterner<'tcx>, _) -> Result<_, _>`

error[E0053]: method `substitution_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:284:9
284 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a Vec<chalk_ir::GenericArg<RustInterner<'_>>>) -> &'a [chalk_ir::GenericArg<RustInterner<'tcx>>]`
               found fn pointer `fn(&RustInterner<'tcx>, &Vec<chalk_ir::GenericArg<RustInterner<'_>>>) -> &[chalk_ir::GenericArg<RustInterner<'tcx>>]`
error[E0053]: method `intern_program_clause` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:291:9
    |
291 |         &self,
291 |         &self,
    |         ^^^^^
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, ProgramClauseData<_>) -> std::boxed::Box<_>`
               found fn pointer `fn(&RustInterner<'tcx>, ProgramClauseData<_>) -> std::boxed::Box<_>`

error[E0053]: method `program_clause_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:298:9
298 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a std::boxed::Box<ProgramClauseData<RustInterner<'_>>>) -> &'a ProgramClauseData<RustInterner<'tcx>>`
               found fn pointer `fn(&RustInterner<'tcx>, &std::boxed::Box<ProgramClauseData<RustInterner<'_>>>) -> &ProgramClauseData<RustInterner<'tcx>>`

error[E0053]: method `intern_program_clauses` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:305:9
305 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, _) -> Result<_, _>`
               found fn pointer `fn(&RustInterner<'tcx>, _) -> Result<_, _>`
error[E0053]: method `program_clauses_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:312:9
    |
312 |         &self,
312 |         &self,
    |         ^^^^^
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a Vec<ProgramClause<RustInterner<'_>>>) -> &'a [ProgramClause<RustInterner<'tcx>>]`
               found fn pointer `fn(&RustInterner<'tcx>, &Vec<ProgramClause<RustInterner<'_>>>) -> &[ProgramClause<RustInterner<'tcx>>]`

error[E0053]: method `intern_quantified_where_clauses` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:319:9
319 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, _) -> Result<_, _>`
               found fn pointer `fn(&RustInterner<'tcx>, _) -> Result<_, _>`

error[E0053]: method `quantified_where_clauses_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:326:9
326 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a Vec<Binders<chalk_ir::WhereClause<RustInterner<'_>>>>) -> &'a [Binders<chalk_ir::WhereClause<RustInterner<'tcx>>>]`
               found fn pointer `fn(&RustInterner<'tcx>, &Vec<Binders<chalk_ir::WhereClause<RustInterner<'_>>>>) -> &[Binders<chalk_ir::WhereClause<RustInterner<'tcx>>>]`

error[E0053]: method `intern_generic_arg_kinds` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:333:9
333 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, _) -> Result<_, _>`
               found fn pointer `fn(&RustInterner<'tcx>, _) -> Result<_, _>`

error[E0053]: method `variable_kinds_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:340:9
340 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a Vec<VariableKind<RustInterner<'_>>>) -> &'a [VariableKind<RustInterner<'tcx>>]`
               found fn pointer `fn(&RustInterner<'tcx>, &Vec<VariableKind<RustInterner<'_>>>) -> &[VariableKind<RustInterner<'tcx>>]`

error[E0053]: method `intern_canonical_var_kinds` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:347:9
347 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, _) -> Result<_, _>`
               found fn pointer `fn(&RustInterner<'tcx>, _) -> Result<_, _>`

error[E0053]: method `canonical_var_kinds_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:354:9
354 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a Vec<WithKind<RustInterner<'_>, chalk_ir::UniverseIndex>>) -> &'a [WithKind<RustInterner<'tcx>, chalk_ir::UniverseIndex>]`
               found fn pointer `fn(&RustInterner<'tcx>, &Vec<WithKind<RustInterner<'_>, chalk_ir::UniverseIndex>>) -> &[WithKind<RustInterner<'tcx>, chalk_ir::UniverseIndex>]`
error[E0053]: method `intern_constraints` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:361:9
    |
361 |         &self,
361 |         &self,
    |         ^^^^^
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, _) -> Result<_, _>`
               found fn pointer `fn(&RustInterner<'tcx>, _) -> Result<_, _>`
error[E0053]: method `constraints_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:368:9
    |
368 |         &self,
368 |         &self,
    |         ^^^^^
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a Vec<InEnvironment<chalk_ir::Constraint<RustInterner<'_>>>>) -> &'a [InEnvironment<chalk_ir::Constraint<RustInterner<'tcx>>>]`
               found fn pointer `fn(&RustInterner<'tcx>, &Vec<InEnvironment<chalk_ir::Constraint<RustInterner<'_>>>>) -> &[InEnvironment<chalk_ir::Constraint<RustInterner<'tcx>>>]`

error[E0053]: method `intern_variances` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:375:9
375 |         &self,
    |         ^^^^^
    |         |
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, _) -> Result<_, _>`
               found fn pointer `fn(&RustInterner<'tcx>, _) -> Result<_, _>`
error[E0053]: method `variances_data` has an incompatible type for trait
   --> compiler/rustc_middle/src/traits/chalk.rs:382:9
    |
382 |         &self,
382 |         &self,
    |         ^^^^^
    |         |
    |         expected struct `RustInterner`, found `&RustInterner<'tcx>`
    |
    |
    = note: expected fn pointer `fn(RustInterner<'tcx>, &'a Vec<chalk_ir::Variance>) -> &'a [chalk_ir::Variance]`
               found fn pointer `fn(&RustInterner<'tcx>, &Vec<chalk_ir::Variance>) -> &[chalk_ir::Variance]`
error[E0308]: mismatched types
   --> compiler/rustc_middle/src/traits/chalk.rs:210:38
    |
    |
210 |         let flags = ty.compute_flags(self);
    |                                      ^^^^ expected struct `RustInterner`, found `&RustInterner<'tcx>`
help: consider dereferencing the borrow
    |
    |
210 |         let flags = ty.compute_flags(*self);

Some errors have detailed explanations: E0053, E0308.
For more information about an error, try `rustc --explain E0053`.
error: could not compile `rustc_middle` due to 30 previous errors
