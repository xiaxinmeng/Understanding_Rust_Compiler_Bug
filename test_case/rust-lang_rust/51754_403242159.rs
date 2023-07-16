rust
macro_rules! idents {
	($($ident:ident)*) => { };
}

idents!{
 a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
 a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
 a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
 a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
 //     ...  65536 copies of "a" in total ...
 a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
 a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
 a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
 a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
}
