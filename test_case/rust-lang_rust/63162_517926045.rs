rust
            let cargo_bin = builder.out.join("bin");
            cargo.env("CARGO_INSTALL_ROOT", &cargo_bin);
            cargo.env("XARGO", cargo_bin.join("xargo"));
