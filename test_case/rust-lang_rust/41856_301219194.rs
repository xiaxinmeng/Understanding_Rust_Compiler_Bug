rust
macro_rules! pathexpr {
    (($($x:tt)*), $p:path, ($($y:tt)*)) => ($($x)* $p $($y)*);
}

mod m1 {
    pub struct A {}
    pub struct B();
    pub struct C;
    pub trait Marker {}
}

trait Tr {
    type A;
    type B;
    type C;
    fn f();
}

struct D;
impl Tr for D {
    type A = m1::A;
    type B = m1::B;
    type C = m1::C;
    fn f() {}
}

// ItemUse
// pathexpr!((use), std::boxed()::Box, (;)); // ERROR from ast_validation

// Visibility
mod m2 {
    macro_rules! foo {
        ($p:path) => { pub(in $p) mod mm {} }
    }
    // foo!(m2()); // ERROR from ast_validation
}

fn main() {
    // QPath::Resolved in PatKind::Struct
    // let pathexpr!((), m1()::A , ({})) = m1::A {}; // ERROR from astconv
    // let pathexpr!((), m1::A() , ({})) = m1::A {}; // ERROR from astconv

    // QPath::Resolved in PatKind::TupleStruct
    // let pathexpr!((), m1()::B , (())) = m1::B(); // ERROR from astconv
    // let pathexpr!((), m1::B() , (())) = m1::B(); // ICE

    // QPath::Resolved in PatKind::Path
    // let pathexpr!((), m1()::C , ()) = m1::C; // ERROR from astconv
    // let pathexpr!((), m1::C() , ()) = m1::C; // ICE

    // QPath::Resolved in ExprPath
    // pathexpr!((), m1()::C, ()); // ERROR from astconv
    // pathexpr!((), m1::C(), ()); // ICE

    // QPath::Resolved in ExprStruct
    // pathexpr!((), m1()::A, ({})); // ERROR from astconv
    // pathexpr!((), m1::A(), ({})); // ERROR from astconv

    // QPath::Resolved in TyPath
    // let _ : m1()::C = m1::C; // ERROR from astconv
    // let _ : m1::C() = m1::C; // ERROR from astconv

    // QPath::TypeRelative in PatKind::Struct
    // let pathexpr!((), D::A(), ({})) = m1::A {}; // ERROR from astconv

    // QPath::TypeRelative in PatKind::TupleStruct
    // let pathexpr!((), D::B(), (())) = m1::B (); // resolution error

    // QPath::TypeRelative in PatKind::Path
    // let pathexpr!((), D::C, ()) = m1::C; // resolution error

    // QPath::TypeRelative in ExprPath
    // pathexpr!((), D::f(), ()); // ICE

    // QPath::TypeRelative in ExprStruct
    // pathexpr!((), D::A(), ({})); // ERROR from astconv

    // QPath::TypeRelative in TyPath
    // let _ : D::C() = m1::C; // ERROR from astconv

    // TraitTyParamBound
    { fn f<X: m1()::Marker>() {} }
    // { fn f<X: m1::Marker()>() {} } // type error

    // TyTraitObject
    let x : &m1()::Marker;
    // let x : &m1::Marker(); // type error
    let x : &(m1::Marker + Sync());

    // ItemDefaultImpl: omitted

    // ItemImpl
    impl m1()::Marker for u32 {}
    // impl m1::Marker() for i32 {} // type error
}
