rust
        #[cfg_attr(feature = "rustc-dep-of-std",
                   link(name = "c", kind = "static-nobundle",
                        cfg(target_feature = "crt-static")))]
        #[cfg_attr(feature = "rustc-dep-of-std",
                   link(name = "c", cfg(not(target_feature = "crt-static"))))]
        extern {}
    