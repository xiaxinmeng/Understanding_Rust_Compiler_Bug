rust
struct Place<'tcx> {
    base: PlaceBase,
    projection: &'tcx [PlaceProjection],
}
enum PlaceProjection {
    Projection(Box<PlaceProjection>),
    Deref,
    Index(..),
    ...
}
