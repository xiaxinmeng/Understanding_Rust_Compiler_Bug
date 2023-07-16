rust
        EntryPointType::None => {
            err_if_attr_found(ctxt, id, sym::unix_sigpipe, "can only be used on `fn main()`");
        }
