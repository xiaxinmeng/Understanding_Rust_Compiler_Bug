rust
trait Trait { type T; }
impl Trait for &'_ &'_ () { type T = (); }

/// Invariant
struct Inv<'lt>(fn(&()) -> &mut &'lt ());

fn transmute_lt<'src : 'src, 'dst : 'dst> (
    s: Inv<'src>,
    // `'src : 'dst` i.e. `'src ⊇ 'dst`
    _dst_implied_smaller: <&'dst &'src () as Trait>::T,
    // `'dst : 'src` i.e. `'dst ⊇ 'src`
    _dst_implied_greater: <&'src &'dst () as Trait>::T,
) -> Inv<'dst> {
    // => `'src = 'dst` => does typecheck
    s
}

/// Typechecks!
fn enlarge<'short, 'long : 'short> (
    it: Inv<'short>,
) -> Inv<'long>
{
    // signature of transmute_lt:
    let f: fn(Inv<'short>, (), ()) -> Inv<'long> = transmute_lt::<'short, 'long>;
    f(it, (), ())
}
