rust
trait Recursive {
    type Recur: Recursive;
}

// trait indirection for the recursive struct field
trait Proxy<'a> {
    type Proxied;
}

// lifetime bound R: 'a not necessary with the natural definition
// ...maybe this hints at something?
impl<'a, R: 'a + Recursive> Proxy<'a> for R {
    type Proxied = Commutes<'a, R>;
}

struct Commutes<'a, R: Recursive> {
    _used: &'a R,
    // NB now needs to be boxed as the size checker kicks in
    hierarchy: Option<Box<<R::Recur as Proxy<'a>>::Proxied>>
}
