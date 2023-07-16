plain
    Checking chalk-solve v0.75.0
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error[E0432]: unresolved import `hygiene::LocalExpnId`
   |
   |
44 | pub use hygiene::{ExpnData, ExpnHash, ExpnId, LocalExpnId, SyntaxContext};
   |                                               ^^^^^^^^^^^ no `LocalExpnId` in `hygiene`
error[E0433]: failed to resolve: use of undeclared type `LocalExpnId`
   --> compiler/rustc_span/src/hygiene.rs:154:45
    |
    |
154 |         if self.krate == LOCAL_CRATE { Some(LocalExpnId::from_raw(self.local_id)) } else { None }
    |                                             ^^^^^^^^^^^ use of undeclared type `LocalExpnId`
error[E0433]: failed to resolve: use of undeclared type `LocalExpnId`
   --> compiler/rustc_span/src/hygiene.rs:758:23
    |
    |
758 |         let expn_id = LocalExpnId::fresh(expn_data, ctx).to_expn_id();
    |                       ^^^^^^^^^^^ use of undeclared type `LocalExpnId`

error[E0412]: cannot find type `LocalExpnId` in this scope
    |
    |
135 | impl ExpnId {
    |     - help: you might be missing a type parameter: `<LocalExpnId>`
...
153 |     pub fn as_local(self) -> Option<LocalExpnId> {


error[E0412]: cannot find type `LocalExpnId` in this scope
    |
    |
159 |     pub fn expect_local(self) -> LocalExpnId {


error[E0412]: cannot find type `LocalExpnId` in this scope
    |
    |
207 | pub struct HygieneData {
    |                       - help: you might be missing a type parameter: `<LocalExpnId>`
...
211 |     local_expn_data: IndexVec<LocalExpnId, Option<ExpnData>>,


error[E0412]: cannot find type `LocalExpnId` in this scope
    |
    |
207 | pub struct HygieneData {
    |                       - help: you might be missing a type parameter: `<LocalExpnId>`
...
212 |     local_expn_hashes: IndexVec<LocalExpnId, ExpnHash>,


error[E0412]: cannot find type `LocalExpnId` in this scope
    |
    |
263 |     fn local_expn_hash(&self, expn_id: LocalExpnId) -> ExpnHash {


error[E0412]: cannot find type `LocalExpnId` in this scope
    |
    |
275 |     fn local_expn_data(&self, expn_id: LocalExpnId) -> &ExpnData {


error[E0412]: cannot find type `LocalExpnId` in this scope
     |
     |
1285 | impl<E: Encoder> Encodable<E> for LocalExpnId {


error[E0412]: cannot find type `LocalExpnId` in this scope
     |
     |
1297 | impl<D: Decoder> Decodable<D> for LocalExpnId {

error[E0283]: type annotations needed
   --> compiler/rustc_span/src/hygiene.rs:239:30
    |
    |
239 |             local_expn_data: IndexVec::from_elem_n(Some(root_data), 1),
    |                              ^^^^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `I`
    = note: cannot satisfy `_: Idx`
    = note: cannot satisfy `_: Idx`
note: required by a bound in `IndexVec::<I, T>::from_elem_n`
    |
    |
541 | impl<I: Idx, T> IndexVec<I, T> {
    |         ^^^ required by this bound in `IndexVec::<I, T>::from_elem_n`
Some errors have detailed explanations: E0283, E0412, E0432, E0433.
For more information about an error, try `rustc --explain E0283`.
error: could not compile `rustc_span` due to 12 previous errors
warning: build failed, waiting for other jobs to finish...
