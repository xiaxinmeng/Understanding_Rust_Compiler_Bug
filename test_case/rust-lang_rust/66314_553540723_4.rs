
2019-11-13T18:34:56.0495322Z     let f: extern "C" fn(*mut i32) = transmute(foo as extern "C" fn(_));
2019-11-13T18:34:56.0495491Z     let f: extern "C" fn(*mut i32) = transmute(foo as usize); // works too
2019-11-13T18:34:56.0495640Z     