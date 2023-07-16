rust
struct Place<'tcx> {
    base: PlaceBase,
    projection: &'tcx ty::List<PlaceProjection>,
}
enum PlaceProjection {
    Projection(Box<PlaceProjection>),
    Deref,
    Index(..),
    ...
}
