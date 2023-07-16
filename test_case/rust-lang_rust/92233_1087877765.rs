plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0277]: the trait bound `HirId: Ord` is not satisfied
    |
    |
131 |     type KeyType = (HirId, u16, u16);
    |                    ^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `HirId`
    |
    = note: required because of the requirements on the impl of `Ord` for `(HirId, u16, u16)`
note: required by a bound in `rustc_data_structures::stable_hasher::ToStableHashKey::KeyType`
    |
    |
206 |     type KeyType: Ord + Sized + HashStable<HCX>;
    |                   ^^^ required by this bound in `rustc_data_structures::stable_hasher::ToStableHashKey::KeyType`

error[E0277]: can't compare `HirId` with `HirId`
     |
     |
77   |   #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Encodable, Decodable)]
...
...
86   |       Stable { hir_id: HirId, attr_index: u16, lint_index: Option<u16> },
     |                ^^^^^^^^^^^^^ no implementation for `HirId < HirId` and `HirId > HirId`
    ::: /checkout/library/core/src/cmp.rs:1172:1
     |
1172 | / pub macro PartialOrd($item:item) {
1173 | |     /* compiler built-in */
1173 | |     /* compiler built-in */
1174 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `HirId`

error[E0277]: the trait bound `HirId: Ord` is not satisfied
    |
    |
77  |   #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Encodable, Decodable)]
...
...
86  |       Stable { hir_id: HirId, attr_index: u16, lint_index: Option<u16> },
    |                ^^^^^^^^^^^^^ the trait `Ord` is not implemented for `HirId`
   ::: /checkout/library/core/src/cmp.rs:860:1
    |
    |
860 | / pub macro Ord($item:item) {
862 | | }
862 | | }
    | |_- in this expansion of `#[derive(Ord)]`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_lint_defs` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
