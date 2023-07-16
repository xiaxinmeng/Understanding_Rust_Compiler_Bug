plain
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
   Compiling rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:502:16
    |
502 |         state: &Self::FlowState,
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 502:16...
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:502:16
    |
502 |         state: &Self::FlowState,
    |                ^^^^^^^^^^^^^^^^
note: ...so that the reference type `&<A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:502:16
    |
502 |         state: &Self::FlowState,


error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:513:16
    |
513 |         state: &Self::FlowState,
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 513:16...
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:513:16
    |
513 |         state: &Self::FlowState,
    |                ^^^^^^^^^^^^^^^^
note: ...so that the reference type `&<A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:513:16
    |
513 |         state: &Self::FlowState,


error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:524:16
    |
524 |         state: &Self::FlowState,
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 524:16...
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:524:16
    |
524 |         state: &Self::FlowState,
    |                ^^^^^^^^^^^^^^^^
note: ...so that the reference type `&<A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:524:16
    |
524 |         state: &Self::FlowState,


error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:536:16
    |
536 |         state: &Self::FlowState,
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 536:16...
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:536:16
    |
536 |         state: &Self::FlowState,
    |                ^^^^^^^^^^^^^^^^
note: ...so that the reference type `&<A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:536:16
    |
536 |         state: &Self::FlowState,


error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:546:16
    |
546 |         state: &Self::FlowState,
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 546:16...
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:546:16
    |
546 |         state: &Self::FlowState,
    |                ^^^^^^^^^^^^^^^^
note: ...so that the reference type `&<A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:546:16
    |
546 |         state: &Self::FlowState,


error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:558:16
    |
558 |         state: &Self::FlowState,
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 558:16...
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:558:16
    |
558 |         state: &Self::FlowState,
    |                ^^^^^^^^^^^^^^^^
note: ...so that the reference type `&<A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/graphviz.rs:558:16
    |
558 |         state: &Self::FlowState,


error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:149:43
    |
149 |     fn reset_to_block_entry(&self, state: &mut Self::FlowState, block: BasicBlock) {
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 149:43...
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:149:43
    |
149 |     fn reset_to_block_entry(&self, state: &mut Self::FlowState, block: BasicBlock) {
    |                                           ^^^^^^^^^^^^^^^^^^^^
note: ...so that the reference type `&mut <A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:149:43
    |
149 |     fn reset_to_block_entry(&self, state: &mut Self::FlowState, block: BasicBlock) {


error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:155:16
    |
155 |         state: &mut Self::FlowState,
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 155:16...
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:155:16
    |
155 |         state: &mut Self::FlowState,
    |                ^^^^^^^^^^^^^^^^^^^^
note: ...so that the reference type `&mut <A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:155:16
    |
155 |         state: &mut Self::FlowState,


error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:164:16
    |
164 |         state: &mut Self::FlowState,
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 164:16...
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:164:16
    |
164 |         state: &mut Self::FlowState,
    |                ^^^^^^^^^^^^^^^^^^^^
note: ...so that the reference type `&mut <A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:164:16
    |
164 |         state: &mut Self::FlowState,


error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:173:16
    |
173 |         state: &mut Self::FlowState,
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 173:16...
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:173:16
    |
173 |         state: &mut Self::FlowState,
    |                ^^^^^^^^^^^^^^^^^^^^
note: ...so that the reference type `&mut <A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:173:16
    |
173 |         state: &mut Self::FlowState,


error[E0311]: the associated type `<A as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:182:16
    |
182 |         state: &mut Self::FlowState,
    |
    |
note: the associated type `<A as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 182:16...
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:182:16
    |
182 |         state: &mut Self::FlowState,
    |                ^^^^^^^^^^^^^^^^^^^^
note: ...so that the reference type `&mut <A as AnalysisDomain<'_>>::Domain` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:182:16
    |
182 |         state: &mut Self::FlowState,


error[E0311]: the associated type `<E as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:229:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
229 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation
    |
    |
note: the associated type `<E as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 229:24...
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:229:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
229 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation
    | |_- in this macro invocation
note: ...so that the reference type `&mut BorrowckAnalyses<<B as AnalysisDomain<'_>>::Domain, <U as AnalysisDomain<'_>>::Domain, <E as AnalysisDomain<'_>>::Domain>` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:229:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
229 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation


error[E0311]: the associated type `<U as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:229:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
229 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation
    |
    |
note: the associated type `<U as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 229:24...
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:229:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
229 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation
    | |_- in this macro invocation
note: ...so that the reference type `&mut BorrowckAnalyses<<B as AnalysisDomain<'_>>::Domain, <U as AnalysisDomain<'_>>::Domain, <E as AnalysisDomain<'_>>::Domain>` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:229:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
229 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation


error[E0311]: the associated type `<B as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:229:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
229 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation
    |
    |
note: the associated type `<B as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 229:24...
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:229:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
229 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation
    | |_- in this macro invocation
note: ...so that the reference type `&mut BorrowckAnalyses<<B as AnalysisDomain<'_>>::Domain, <U as AnalysisDomain<'_>>::Domain, <E as AnalysisDomain<'_>>::Domain>` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:229:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
229 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation


error[E0311]: the associated type `<E as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:237:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
237 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation
    |
    |
note: the associated type `<E as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 237:24...
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:237:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
237 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation
    | |_- in this macro invocation
note: ...so that the reference type `&mut BorrowckAnalyses<<B as AnalysisDomain<'_>>::Domain, <U as AnalysisDomain<'_>>::Domain, <E as AnalysisDomain<'_>>::Domain>` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:237:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
237 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation


error[E0311]: the associated type `<U as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:237:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
237 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation
    |
    |
note: the associated type `<U as AnalysisDomain<'_>>::Domain` must be valid for the anonymous lifetime defined on the method body at 237:24...
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:237:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
237 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation
    | |_- in this macro invocation
note: ...so that the reference type `&mut BorrowckAnalyses<<B as AnalysisDomain<'_>>::Domain, <U as AnalysisDomain<'_>>::Domain, <E as AnalysisDomain<'_>>::Domain>` does not outlive the data it points at
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:237:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
237 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
277 | 
278 | / impl_visitable! {
279 | |     BorrowckAnalyses { borrows: B, uninits: U, ever_inits: E }
    | |_- in this macro invocation


error[E0311]: the associated type `<B as AnalysisDomain<'_>>::Domain` may not live long enough
   --> compiler/rustc_mir/src/dataflow/framework/visitor.rs:237:24
210 | / macro_rules! impl_visitable {
211 | |     ( $(
211 | |     ( $(
212 | |         $T:ident { $( $field:ident : $A:ident ),* $(,)? }
213 | |     )* ) => { $(
...   |
237 | |                 state: &mut Self::FlowState,
...   |
275 | |     )* }
276 | | }
276 | | }
    | |_- in this expansion of `impl_visitable!`
