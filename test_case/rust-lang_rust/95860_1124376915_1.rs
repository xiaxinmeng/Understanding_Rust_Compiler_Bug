rust
macro_rules! define {
    ($vis:vis fn $f:ident() $(-> $Ret:ty)?) => {
        $vis fn $f() $(-> ${ignore(Ret)} u32)? { 0 }
    }
}
