
rustdoc --no-defaults lib.rs                          # The bug does not occur
rustdoc --no-defaults --passes collapse-docs lib.rs   # The bug occurs
