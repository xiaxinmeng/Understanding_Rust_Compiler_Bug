
 error[E0277]: the trait bound `u64: graph::geometry::Geometry` is not satisfied
   --> /.../plexus-0.0.1/src/graph/mesh.rs:90:1
    |
 90 | / pub struct Mesh<G = u64>
 91 | | where
 92 | |     G: Geometry,
 93 | | {
 ...  |
 96 | |     pub(super) faces: Storage<FaceKey, Face<G::Face>>,
 97 | | }
    | |_^ the trait `graph::geometry::Geometry` is not implemented for `u64`
    |
 note: required by `graph::geometry::Geometry`
   --> /.../plexus-0.0.1/src/graph/geometry/mod.rs:8:1
    |
 8  | pub trait Geometry: Sized {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
