rust
 if let Err(_) = rustc_demangle::try_demangle(&mangled) {
                bug!("demangle error: {:?}\n{}", instance, mangled);
            }
