
error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/traits/chalk.rs:77:5
    |
77  |     type DefId = DefId;
    |     ^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
    |
note: required by a bound in `chalk_ir::interner::Interner::DefId`
   --> /Users/user/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/interner.rs:191:37
    |
191 |     type DefId: Debug + Copy + Eq + Ord + Hash;
    |                                     ^^^ required by this bound in `chalk_ir::interner::Interner::DefId`

error[E0277]: the trait bound `adt::AdtDef: Ord` is not satisfied
   --> compiler/rustc_middle/src/traits/chalk.rs:78:5
    |
78  |     type InternedAdtId = &'tcx AdtDef;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `adt::AdtDef`
    |
    = note: required because of the requirements on the impl of `Ord` for `&'tcx adt::AdtDef`
note: required by a bound in `chalk_ir::interner::Interner::InternedAdtId`
   --> /Users/user/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/interner.rs:194:45
    |
194 |     type InternedAdtId: Debug + Copy + Eq + Ord + Hash;
    |                                             ^^^ required by this bound in `chalk_ir::interner::Interner::InternedAdtId`
