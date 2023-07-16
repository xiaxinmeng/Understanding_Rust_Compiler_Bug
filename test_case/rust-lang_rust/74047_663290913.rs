
pub async fn connect()
 ->
    /*impl Trait*/ ::std::future::from_generator(move |mut _task_context| {
        {
            let _t =
                {
                    let stream:
                            MyStream =
                        match ::std::ops::Try::into_result(OtherStream.try_into())
                            {
                            ::std::result::Result::Err(err) // `err` replaced with `_` here!
                            =>

                                #[allow(unreachable_code)]
                                return ::std::ops::Try::from_error(::std::convert::From::from(err)),
                            ::std::result::Result::Ok(val)
                            =>

                                #[allow(unreachable_code)]
                                val,
                        };
                    Ok(stream)
                };
            _t
        }
    })
