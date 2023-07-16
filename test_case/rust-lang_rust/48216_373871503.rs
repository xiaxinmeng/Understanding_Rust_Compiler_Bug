rust
pub macro outer_use1($Rename: ident) {
    use m::Tr as $Rename;
}
pub macro outer_use2($Tr: ident) {
    use m::$Tr;
}
pub macro outer_use3($Tr: path) {
    pub use $Tr;
}

outer_use1!(Rename);
outer_use2!(Tr);
outer_use3!(m::Tr);

 // still a "no method named `f`" error after expanding all the macros above
S.f();
