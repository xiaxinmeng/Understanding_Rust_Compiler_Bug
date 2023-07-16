
   = note: If the `input_binds` field serves some purpose (e.g. it has a drop effect, 
      or is used by foreign code), then you can prepend underscore to its name:

26 |   _input_binds: Option<Binds>,

    The leading underscore is a signal to a reader that the field might not be read 
    explicitly by any of the Rust code, but still serves a purpose.
