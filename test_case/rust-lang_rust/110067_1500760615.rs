
"/usr/bin/rustc" "--crate-name" "core" "--edition=2021" "library/core/src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--diagnostic-width=80" "--crate-type" "lib" "--emit=dep-info,metadata,link" "--cfg=bootstrap" "-Zunstable-options" "--check-cfg=values(bootstrap)"
