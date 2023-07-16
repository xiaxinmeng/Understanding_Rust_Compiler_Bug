 bash
rustc src/lib.rs --crate-name main --target i686-unknown-linux-gnu --opt-level=3 -Z lto -Z no-landing-pads
