rust
        impl Default for Providers {
            fn default() -> Self {
                Providers {
                    $($name: |tcx, key| bug!(
                        "`tcx.{}({:?})` no provider available for {} crate; perhaps the `{}` query was never assigned a provider function",
                         stringify!($name),
                         key,
                         if tcx.is_local() { "local" } else { "external" },
                         stringify!($name),
                    ),)*
                }
            }
        }
