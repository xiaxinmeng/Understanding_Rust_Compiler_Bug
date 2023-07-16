

error[E0308]: mismatched types
   --> dag-store/src/client.rs:202:17
    |
202 | /                 <Compose<MerkleLayer<PartiallyApplied>, F> as FunctorExt>::expand_and_collapse(
203 | |                         header,
204 | |                         |header: Header| -> <Compose<MerkleLayer<PartiallyApplied>, F> as Functor>::Layer<
205 | |                             Header,
...   |
217 | |                         },
218 | |                     )
    | |_____________________^ expected enum `MerkleLayer`, found struct `Header`
    |
    = note: expected enum `MerkleLayer<Fix<Compose<F, MerkleLayer<PartiallyApplied>>>>`
             found struct `dag_store_types::types::domain::Header`
help: try wrapping the expression in `client::MerkleLayer::Remote`
    |
202 ~                 client::MerkleLayer::Remote(<Compose<MerkleLayer<PartiallyApplied>, F> as FunctorExt>::expand_and_collapse(
203 |                         header,
  ...
217 |                         },
218 ~                     ))
    |

error[E0631]: type mismatch in closure arguments
   --> dag-store/src/client.rs:202:17
    |
202 |                 <Compose<MerkleLayer<PartiallyApplied>, F> as FunctorExt>::expand_and_collapse(
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected due to this
...
213 |                         |layer: MerkleLayer<<F as Functor>::Layer<MerkleLayer<Fix<Compose<F, MerkleLayer<PartiallyApplied>>>>>>| -> MerkleLayer<Fix<Compose<F, MerkleLayer<PartiallyApplied>>>>   {
    |                         ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- found signature defined here
    |
    = note: expected closure signature `fn(MerkleLayer<<F as Functor>::Layer<dag_store_types::types::domain::Header>>) -> _`
               found closure signature `fn(MerkleLayer<<F as Functor>::Layer<MerkleLayer<Fix<Compose<F, MerkleLayer<PartiallyApplied>>>>>>) -> _`
note: required by a bound in `recursion_schemes::functor::FunctorExt::expand_and_collapse`
   --> /home/inanna/.cargo/registry/src/github.com-1ecc6299db9ec823/recursion-schemes-0.1.2/src/functor.rs:60:30
    |
60  |         collapse_layer: impl FnMut(<Self as Functor>::Layer<Out>) -> Out,
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `FunctorExt::expand_and_collapse`

wa
