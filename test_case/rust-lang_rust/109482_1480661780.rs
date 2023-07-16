rust
trait Trait {
    type Assoc;
}

impl<T: 'static> Trait for Box<T> {
    type Assoc = &'static T; 
}

impl<T> Trait for (T, <Box<T> as Trait>::Assoc) {
    type Assoc = &'static T; // We can still assume T: 'static by wfcheck
}
