rust
struct CurrentVec<T> {
    elements: Box<T>
}

struct SmartRepresentationVec<T> {
    elements: <T as Smart>::Representation,
}

trait Smart {
    type Representation;
}

impl<T> Smart for T {
    type Representation = Box<T>;
}

// This compiles!
fn change_lifetime1<'a, T>(input: CurrentVec<&'static T>) -> CurrentVec<&'a T> {
    input
}

// This does not! The SmartRepresentationVec is invariant in its parameter.
fn change_lifetime2<'a, T>(input: SmartRepresentationVec<&'static T>) -> SmartRepresentationVec<&'a T> {
    input
}
