
        logo = {
            if layout.logo.is_empty() {
                format!(
                    "<a href='{root}{path}index.html'>\
                     <div class='logo-container rust-logo'>\
                     <img src='{static_root_path}rust-logo{suffix}.png' alt='logo'></div></a>",
                    root = page.root_path,
                    path = ensure_trailing_slash(&layout.krate),
                    static_root_path = static_root_path,
                    suffix = page.resource_suffix
                )
            } else {
                format!(
                    "<a href='{root}{path}index.html'>\
                     <div class='logo-container'><img src='{logo}' alt='logo'></div></a>",
                    root = page.root_path,
                    path = ensure_trailing_slash(&layout.krate),
                    logo = layout.logo
                )
            }
        },
