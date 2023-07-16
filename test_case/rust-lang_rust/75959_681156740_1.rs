
    let utf8_method = std::mem::transmute::<_, unsafe extern "C" fn(_, _) -> _>(objc_msgSend as unsafe extern "C" fn());
