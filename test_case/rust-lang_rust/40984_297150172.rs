rust
macro_rules! thread_local {
    // terminate recursion
    (@ [] -> $_x:tt) => {};
    
    // munch one attribute
    (@ [#[$attr:meta] $($rest:tt)*] -> [$(#[$attrs:meta])*]) => {
        thread_local!(@ [$($rest)*] -> [$(#[$attrs])* #[$attr]]);
    };
    
    // finish a static with no trailing semicolon (therefore there are no more)
    (@ [$vis:vis static $name:ident: $typ:ty = $init:expr] -> [$(#[$attrs:meta])*]) => {
        __thread_local_inner!(($(#[$attrs])*) $vis, $name, $typ, $init);
    };
    
    // finish a static with trailing semicolon and continue to parse more
    (@ [$vis:vis static $name:ident: $typ:ty = $init:expr; $($rest:tt)*] -> [$(#[$attrs:meta])*]) => {
        __thread_local_inner!(($(#[$attrs])*) $vis, $name, $typ, $init);
        thread_local!(@ [$($rest)*] -> []);
    };
    
    // public entry point
    ($($t:tt)*) => {
        thread_local!(@ [$($t)*] -> []);
    }
}
