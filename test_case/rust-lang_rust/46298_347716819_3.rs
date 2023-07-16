wasm
 (func $0 (; 0 ;) (type $0) (param $var$0 f64) (result i32)
  (local $var$1 i64)
  (block $label$1
   (block $label$2
    (br_if $label$2
     (i32.eqz
      (f64.lt
       (get_local $var$0)
       (f64.const 18446744073709551615)
      )
     )
    )
    (set_local $var$1
     (i64.trunc_u/f64
      (get_local $var$0)
     )
    )
    (br $label$1)
   )
   (set_local $var$1
    (i64.const 0)
   )
  )
  (i64.ne
   (get_local $var$1)
   (i64.const 0)
  )
)
