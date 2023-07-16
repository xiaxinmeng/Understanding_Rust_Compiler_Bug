
macro_rules! ffi {
    // $Crt is the C function's return type.  It must be possible to
    // convert an Error value v of type $rt to a value of type $Crt by doing:
    // $Crt::from($rt).
    //
    // $ok is the value (of type $rt) to map Ok to.
    (fn $f:ident($($v:ident: $t:ty),*)
        -> Result<$rt:ty, $et:ty>
        -> ($Crt:ty; $rt_to_crt:expr; $err_to_crt: expr)
        $body:block
     ) =>
    {
        // The wrapper.  It calls $f and turns the result into an
        // error code.
        #[allow(unused)]
        #[no_mangle] pub extern "C"
        fn $f($($v: $t),*) -> $Crt {
            // The actual function.
            let inner = |$($v: $t),*| -> std::result::Result<$rt, $et> { $body };

            match inner($($v,)*) {
                Ok(v) => {
                    // XXX: Replace this line with a panic! and the ICE goes away.
                    let rt: $Crt = $rt_to_crt(v);
                    rt
                    // panic!();
                }
                Err(err) => {
                    let rt: $Crt = $err_to_crt(err);
                    rt
                }
            }
        }
    }
}
