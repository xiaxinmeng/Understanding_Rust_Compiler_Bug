bash
#!/bin/bash

cat > Cargo.toml << EOF
[package]
name = "abc"
version = "0.1.0"

[lib]
crate-type = ["staticlib"]
path = "lib.rs"

[dependencies]
other = { path = "./other" }

[profile.release]
lto = true
EOF

cat > lib.rs << EOF
extern crate other;
EOF

mkdir other
cat > other/Cargo.toml << EOF
[package]
name = "other"
version = "0.1.0"

[lib]
path = "lib.rs"

[dependencies]
EOF

cat > other/lib.rs << EOF

pub struct FloatVal {
    integral: &'static str,
    exponent: Option<&'static str>,
}

impl FloatVal {
    pub fn fmt(&self, f: &mut ::std::fmt::Formatter) {
        match *self {
            FloatVal {
                ref integral,
                ref exponent,
            } => {
                let debug_trait_builder = &mut ::std::fmt::Formatter::debug_struct(f, "FloatVal");
                let _ = ::std::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "integral",
                    &&(*integral),
                );
                let _ = ::std::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "exponent",
                    &&(*exponent),
                );
            }
        };
    }
}
EOF
