rust

struct Place {
    base: PlaceBase,
    projection: PlaceProjection,
}
enum PlaceProjection {
    Base,
    Projection(Box<PlaceProjection>),
    Deref,
    Index(..),
    ...
}
