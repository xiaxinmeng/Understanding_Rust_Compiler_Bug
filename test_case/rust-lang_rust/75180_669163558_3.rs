
error: lifetime may not live long enough
  --> examples\custom_handler.rs:65:60
   |
65 |         let errors = iter::successors(Some(error), |error| error.source());
   |                                                     ------ ^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
   |                                                     |    |
   |                                                     |    return type of closure is std::option::Option<&'2 dyn std::error::Error>
   |                                                     has type `&'1 &dyn std::error::Error`
