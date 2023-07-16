rust
        impl Default for Providers {
            fn default() -> Self {
                Providers {
                    $($name: |_, key| bug!(
                        "`tcx.{}({:?})` unsupported for {} crate; \
                         perhaps the `{}` query was never assigned a provider function. Queries can be either made to the local crate, or the external crate. This error means you tried to use it for one that's not supported.", 
                        stringify!($name),
                        key,
                        if key.query_crate_is_local() { "local" } else { "external" } ,
                        stringify!($name),
                    ),)*
                }
            }
        }
