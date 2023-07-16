plain
   Compiling chalk-ir v0.55.0
error[E0786]: found invalid metadata files for crate `chalk_derive`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:13:5
   |
13 | use chalk_derive::{Fold, HasInterner, SuperVisit, Visit, Zip};
   |
   |
   = note: no `.rustc` section in '/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libchalk_derive-c5e9a09a719945f3.so'

error[E0432]: unresolved imports `super::Fold`, `crate::Visit`, `crate::SuperVisit`, `crate::Visit`, `crate::Visit`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/fold/shift.rs:3:5
3  | use super::Fold;
   |     ^^^^^^^^^^^
   |
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit/binder_impls.rs:7:82
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit/binder_impls.rs:7:82
   |
7  | use crate::{Binders, Canonical, ControlFlow, DebruijnIndex, FnPointer, Interner, Visit, Visitor};
   |
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit/boring_impls.rs:11:75
   |
   |
11 |     QuantifiedWhereClauses, QuantifierKind, Safety, Scalar, Substitution, SuperVisit, TraitId,
   |                                                                           ^^^^^^^^^^
12 |     UintTy, UniverseIndex, Visit, Visitor,
   |
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit/visitors.rs:3:61
   |
   |
3  | use crate::{BoundVar, ControlFlow, DebruijnIndex, Interner, Visit, Visitor};


error: cannot find derive macro `Fold` in this scope
    |
    |
149 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
    |
    |
note: `Fold` is imported here, but it is only a trait, without a derive macro
    |
    |
11  | use crate::fold::{Fold, Folder, Subst, SuperFold};

error: cannot find derive macro `Visit` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:149:44
    |
    |
149 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
    |
    |
note: `Visit` is imported here, but it is only a trait, without a derive macro
    |
    |
12  | use crate::visit::{ControlFlow, SuperVisit, Visit, VisitExt, Visitor};


error: cannot find derive macro `HasInterner` in this scope
    |
    |
149 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
    |
    |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
    |
    |
70  | use interner::{HasInterner, Interner};


error: cannot find derive macro `Fold` in this scope
    |
    |
199 | #[derive(Clone, Debug, PartialEq, Eq, Hash, Fold, Visit)]
    |
    |
note: `Fold` is imported here, but it is only a trait, without a derive macro
    |
    |
11  | use crate::fold::{Fold, Folder, Subst, SuperFold};

error: cannot find derive macro `Visit` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:199:51
    |
    |
199 | #[derive(Clone, Debug, PartialEq, Eq, Hash, Fold, Visit)]
    |
    |
note: `Visit` is imported here, but it is only a trait, without a derive macro
    |
    |
12  | use crate::visit::{ControlFlow, SuperVisit, Visit, VisitExt, Visitor};


error: cannot find derive macro `HasInterner` in this scope
    |
    |
409 | #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
    |
    |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
    |
    |
70  | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
    |
    |
665 | #[derive(Clone, PartialEq, Eq, Hash, HasInterner)]
    |
    |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
    |
    |
70  | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
    |
    |
721 | #[derive(Clone, PartialEq, Eq, Hash, HasInterner)]
    |
    |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
    |
    |
70  | use interner::{HasInterner, Interner};


error: cannot find derive macro `Fold` in this scope
     |
     |
1101 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `Fold` is imported here, but it is only a trait, without a derive macro
     |
     |
11   | use crate::fold::{Fold, Folder, Subst, SuperFold};

error: cannot find derive macro `Visit` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:1101:44
     |
     |
1101 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `Visit` is imported here, but it is only a trait, without a derive macro
     |
     |
12   | use crate::visit::{ControlFlow, SuperVisit, Visit, VisitExt, Visitor};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1101 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1156 | #[derive(Clone, Copy, PartialEq, Eq, Hash, HasInterner, Debug)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1164 | #[derive(Clone, PartialEq, Eq, Hash, HasInterner, Fold, Visit)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `Fold` in this scope
     |
     |
1164 | #[derive(Clone, PartialEq, Eq, Hash, HasInterner, Fold, Visit)]
     |
     |
note: `Fold` is imported here, but it is only a trait, without a derive macro
     |
     |
11   | use crate::fold::{Fold, Folder, Subst, SuperFold};

error: cannot find derive macro `Visit` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:1164:57
     |
     |
1164 | #[derive(Clone, PartialEq, Eq, Hash, HasInterner, Fold, Visit)]
     |
     |
note: `Visit` is imported here, but it is only a trait, without a derive macro
     |
     |
12   | use crate::visit::{ControlFlow, SuperVisit, Visit, VisitExt, Visitor};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1171 | #[derive(Clone, PartialEq, Eq, Hash, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1206 | #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1260 | #[derive(Clone, PartialEq, Eq, Hash, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1269 | #[derive(Clone, PartialEq, Eq, Hash, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1292 | #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1306 | #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1364 | #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1480 | #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};

error: cannot find derive macro `Visit` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:1552:38
     |
     |
1552 | #[derive(Clone, PartialEq, Eq, Hash, Visit, Fold, Zip)]
     |
     |
note: `Visit` is imported here, but it is only a trait, without a derive macro
     |
     |
12   | use crate::visit::{ControlFlow, SuperVisit, Visit, VisitExt, Visitor};


error: cannot find derive macro `Fold` in this scope
     |
     |
1552 | #[derive(Clone, PartialEq, Eq, Hash, Visit, Fold, Zip)]
     |
     |
note: `Fold` is imported here, but it is only a trait, without a derive macro
     |
     |
11   | use crate::fold::{Fold, Folder, Subst, SuperFold};


error: cannot find derive macro `Zip` in this scope
     |
     |
1552 | #[derive(Clone, PartialEq, Eq, Hash, Visit, Fold, Zip)]


error: cannot find derive macro `Fold` in this scope
     |
     |
1637 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner, Zip)]
     |
     |
note: `Fold` is imported here, but it is only a trait, without a derive macro
     |
     |
11   | use crate::fold::{Fold, Folder, Subst, SuperFold};

error: cannot find derive macro `Visit` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:1637:44
     |
     |
1637 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner, Zip)]
     |
     |
note: `Visit` is imported here, but it is only a trait, without a derive macro
     |
     |
12   | use crate::visit::{ControlFlow, SuperVisit, Visit, VisitExt, Visitor};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1637 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner, Zip)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `Zip` in this scope
     |
     |
1637 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner, Zip)]


error: cannot find derive macro `Fold` in this scope
     |
     |
1663 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `Fold` is imported here, but it is only a trait, without a derive macro
     |
     |
11   | use crate::fold::{Fold, Folder, Subst, SuperFold};

error: cannot find derive macro `Visit` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:1663:44
     |
     |
1663 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `Visit` is imported here, but it is only a trait, without a derive macro
     |
     |
12   | use crate::visit::{ControlFlow, SuperVisit, Visit, VisitExt, Visitor};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1663 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `Fold` in this scope
     |
     |
1685 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `Fold` is imported here, but it is only a trait, without a derive macro
     |
     |
11   | use crate::fold::{Fold, Folder, Subst, SuperFold};

error: cannot find derive macro `Visit` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:1685:44
     |
     |
1685 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `Visit` is imported here, but it is only a trait, without a derive macro
     |
     |
12   | use crate::visit::{ControlFlow, SuperVisit, Visit, VisitExt, Visitor};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1685 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `Fold` in this scope
     |
     |
1701 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `Fold` is imported here, but it is only a trait, without a derive macro
     |
     |
11   | use crate::fold::{Fold, Folder, Subst, SuperFold};

error: cannot find derive macro `Visit` in this scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/lib.rs:1701:44
     |
     |
1701 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `Visit` is imported here, but it is only a trait, without a derive macro
     |
     |
12   | use crate::visit::{ControlFlow, SuperVisit, Visit, VisitExt, Visitor};


error: cannot find derive macro `HasInterner` in this scope
     |
     |
1701 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |
     |
note: `HasInterner` is imported here, but it is only a trait, without a derive macro
     |
     |
70   | use interner::{HasInterner, Interner};


error: cannot find derive macro `Fold` in this scope
     |
     |
1738 | #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner, Zip)]
     |
     |
note: `Fold` is imported here, but it is only a trait, without a derive macro
     |
     |
11   | use crate::fold::{Fold, Folder, Subst, SuperFold};

error: cannot find derive macro `Visit` in this scope
