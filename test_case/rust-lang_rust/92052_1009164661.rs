rust
    let id = cx.derive_id(match i.inner_impl().trait_ {
        Some(ref t) => {
            if is_on_foreign_type {
                get_id_for_impl_on_foreign_type(&i.inner_impl().for_, t, cx)
            } else {
                format!("impl-{}", small_url_encode(format!("{:#}", t.print(cx))))
            }
        }
        None => "impl".to_string(),
    });
