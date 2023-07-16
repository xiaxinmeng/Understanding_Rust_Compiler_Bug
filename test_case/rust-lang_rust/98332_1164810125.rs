
StatementKind::Intrinsic {
     inputs: Vec<Place>,
    output: Option<Place>,
    intrinsic: IntrinsicKind,
}

#[non_exhaustive]
enum IntrinsicKind { Assume, ... }
