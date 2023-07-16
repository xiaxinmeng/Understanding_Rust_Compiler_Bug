
macro_rules! map {
    ($id:ident: $T:ty {
        $($k: expr => $v: expr),*
    }) => {
        // Need a way to specify mutability here!
        // Not sure how
        let mut $id = {
            use std::default::Default;

            let mut _thing: $T = Default::default();
            $( _thing.insert($k, $v); )*
            _thing
        }
    }
}
