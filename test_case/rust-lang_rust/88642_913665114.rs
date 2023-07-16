plain
   Compiling tracing-attributes v0.1.13
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.55.0
   Compiling chalk-ir v0.55.0
error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
    |
    |
154 |     pub clauses: ProgramClauses<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
    |
    |
202 |     pub environment: Environment<G::Interner>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
    |
    |
411 |     interned: I::InternedType,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
    |
    |
668 |     pub kind: TyKind<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
    |
    |
725 |     Adt(AdtId<I>, Substitution<I>),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1104 |     pub bounds: Binders<QuantifiedWhereClauses<I>>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1159 |     pub abi: I::FnAbi,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1165 | pub struct FnSubst<I: Interner>(pub Substitution<I>);


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
1174 |     pub num_binders: usize,
     |                      ^^^^^


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1208 |     interned: I::InternedConst,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
1263 |     pub ty: Ty<I>,
     |             ^^


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1272 |     BoundVar(BoundVar),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1295 |     pub interned: I::InternedConcreteConst,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1308 |     interned: I::InternedLifetime,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1367 |     BoundVar(BoundVar),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1482 |     interned: I::InternedGenericArg,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1555 |     Ty(Ty<I>),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1640 |     Projection(ProjectionTy<I>),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1666 |     pub associated_ty_id: AssocTypeId<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1688 |     pub opaque_ty_id: OpaqueTyId<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1704 |     pub trait_id: TraitId<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1741 |     pub a: Lifetime<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
1752 |     pub ty: Ty<I>,
     |             ^^


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1768 |     Implemented(TraitRef<I>),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1801 |     Trait(TraitRef<I>),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1837 |     Trait(TraitRef<I>),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
1866 |     Holds(WhereClause<I>),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
2025 |     pub a: GenericArg<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
2035 |     pub a: Ty<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
2048 |     pub alias: AliasTy<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
2063 |     pub alias: AliasTy<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
2326 |     pub consequence: DomainGoal<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
2360 | pub struct ProgramClauseData<I: Interner>(pub Binders<ProgramClauseImplication<I>>);


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
2395 |     interned: I::InternedProgramClause,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
2509 |     interned: I::InternedGoal,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
2613 |     Quantified(QuantifierKind, Binders<Goal<I>>),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
2695 |     LifetimeOutlives(Lifetime<I>, Lifetime<I>),


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
2902 | / macro_rules! interned_slice_common {
2902 | / macro_rules! interned_slice_common {
2903 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2904 | |         /// List of interned elements.
2905 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2906 | |         pub struct $seq<I: Interner> {
2907 | |             interned: I::$interned,
...    |
2946 | |     };
2947 | | }
2947 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2949 | / macro_rules! interned_slice {
2949 | / macro_rules! interned_slice {
2950 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2951 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2952 | |
...    |
2983 | |     };
2984 | | }
2984 | | }
     | |_- in this expansion of `interned_slice!` (#1)
2986 | / interned_slice!(
2986 | / interned_slice!(
2987 | |     QuantifiedWhereClauses,
2988 | |     quantified_where_clauses_data => QuantifiedWhereClause<I>,
2989 | |     intern_quantified_where_clauses => InternedQuantifiedWhereClauses
     | |__- in this macro invocation (#1)


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
2902 | / macro_rules! interned_slice_common {
2902 | / macro_rules! interned_slice_common {
2903 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2904 | |         /// List of interned elements.
2905 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2906 | |         pub struct $seq<I: Interner> {
2907 | |             interned: I::$interned,
...    |
2946 | |     };
2947 | | }
2947 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2949 | / macro_rules! interned_slice {
2949 | / macro_rules! interned_slice {
2950 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2951 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2952 | |
...    |
2983 | |     };
2984 | | }
2984 | | }
     | |_- in this expansion of `interned_slice!` (#1)
2992 | / interned_slice!(
2992 | / interned_slice!(
2993 | |     ProgramClauses,
2994 | |     program_clauses_data => ProgramClause<I>,
2995 | |     intern_program_clauses => InternedProgramClauses
     | |__- in this macro invocation (#1)


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
2902 | / macro_rules! interned_slice_common {
2902 | / macro_rules! interned_slice_common {
2903 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2904 | |         /// List of interned elements.
2905 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2906 | |         pub struct $seq<I: Interner> {
2907 | |             interned: I::$interned,
...    |
2946 | |     };
2947 | | }
2947 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2949 | / macro_rules! interned_slice {
2949 | / macro_rules! interned_slice {
2950 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2951 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2952 | |
...    |
2983 | |     };
2984 | | }
2984 | | }
     | |_- in this expansion of `interned_slice!` (#1)
2998 | / interned_slice!(
2999 | |     VariableKinds,
2999 | |     VariableKinds,
3000 | |     variable_kinds_data => VariableKind<I>,
3001 | |     intern_generic_arg_kinds => InternedVariableKinds
     | |__- in this macro invocation (#1)


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
2902 | / macro_rules! interned_slice_common {
2902 | / macro_rules! interned_slice_common {
2903 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2904 | |         /// List of interned elements.
2905 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2906 | |         pub struct $seq<I: Interner> {
2907 | |             interned: I::$interned,
...    |
2946 | |     };
2947 | | }
2947 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2949 | / macro_rules! interned_slice {
2949 | / macro_rules! interned_slice {
2950 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2951 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2952 | |
...    |
2983 | |     };
2984 | | }
2984 | | }
     | |_- in this expansion of `interned_slice!` (#1)
3004 | / interned_slice!(
3005 | |     CanonicalVarKinds,
3005 | |     CanonicalVarKinds,
3006 | |     canonical_var_kinds_data => CanonicalVarKind<I>,
3007 | |     intern_canonical_var_kinds => InternedCanonicalVarKinds
     | |__- in this macro invocation (#1)


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
2902 | / macro_rules! interned_slice_common {
2902 | / macro_rules! interned_slice_common {
2903 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2904 | |         /// List of interned elements.
2905 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2906 | |         pub struct $seq<I: Interner> {
2907 | |             interned: I::$interned,
...    |
2946 | |     };
2947 | | }
2947 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2949 | / macro_rules! interned_slice {
2949 | / macro_rules! interned_slice {
2950 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2951 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2952 | |
...    |
2983 | |     };
2984 | | }
2984 | | }
     | |_- in this expansion of `interned_slice!` (#1)
...
3010 |   interned_slice!(Goals, goals_data => Goal<I>, intern_goals => InternedGoals);


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
2902 | / macro_rules! interned_slice_common {
2902 | / macro_rules! interned_slice_common {
2903 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2904 | |         /// List of interned elements.
2905 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2906 | |         pub struct $seq<I: Interner> {
2907 | |             interned: I::$interned,
...    |
2946 | |     };
2947 | | }
2947 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2949 | / macro_rules! interned_slice {
2949 | / macro_rules! interned_slice {
2950 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2951 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2952 | |
...    |
2983 | |     };
2984 | | }
2984 | | }
     | |_- in this expansion of `interned_slice!` (#1)
3012 | / interned_slice!(
3013 | |     Constraints,
3013 | |     Constraints,
3014 | |     constraints_data => InEnvironment<Constraint<I>>,
3015 | |     intern_constraints => InternedConstraints
     | |__- in this macro invocation (#1)


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
2902 | / macro_rules! interned_slice_common {
2902 | / macro_rules! interned_slice_common {
2903 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2904 | |         /// List of interned elements.
2905 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2906 | |         pub struct $seq<I: Interner> {
2907 | |             interned: I::$interned,
...    |
2946 | |     };
2947 | | }
2947 | | }
     | |_- in this expansion of `interned_slice_common!` (#2)
2949 | / macro_rules! interned_slice {
2949 | / macro_rules! interned_slice {
2950 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2951 | |         interned_slice_common!($seq, $data => $elem, $intern => $interned);
2952 | |
...    |
2983 | |     };
2984 | | }
2984 | | }
     | |_- in this expansion of `interned_slice!` (#1)
3018 | / interned_slice!(
3019 | |     Substitution,
3019 | |     Substitution,
3020 | |     substitution_data => GenericArg<I>,
3021 | |     intern_substitution => InternedSubstitution
     | |__- in this macro invocation (#1)


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
2902 | / macro_rules! interned_slice_common {
2902 | / macro_rules! interned_slice_common {
2903 | |     ($seq:ident, $data:ident => $elem:ty, $intern:ident => $interned:ident) => {
2904 | |         /// List of interned elements.
2905 | |         #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
2906 | |         pub struct $seq<I: Interner> {
2907 | |             interned: I::$interned,
...    |
2946 | |     };
2947 | | }
     | |_- in this expansion of `interned_slice_common!`
     | |_- in this expansion of `interned_slice_common!`
...
3024 | / interned_slice_common!(
3025 | |     Variances,
3026 | |     variances_data => Variance,
3027 | |     intern_variance => InternedVariances
     | |__- in this macro invocation


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
3068 |     pub subst: Substitution<I>,


error: expected one of: `for`, parentheses, `fn`, `extern`, `<`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime
     |
     |
3080 |     pub subst: Substitution<I>,

error[E0204]: the trait `Copy` may not be implemented for this type
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:1109:19
     |
     |
1104 |     pub bounds: Binders<QuantifiedWhereClauses<I>>,
     |     ---------------------------------------------- this field does not implement `Copy`
...
1109 | impl<I: Interner> Copy for DynTy<I>

error[E0204]: the trait `Copy` may not be implemented for this type
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:2644:19
     |
     |
2613 |     Quantified(QuantifierKind, Binders<Goal<I>>),
     |                                ---------------- this field does not implement `Copy`
...
2644 | impl<I: Interner> Copy for GoalData<I>

For more information about this error, try `rustc --explain E0204`.
error: could not compile `chalk-ir` due to 49 previous errors
warning: build failed, waiting for other jobs to finish...
