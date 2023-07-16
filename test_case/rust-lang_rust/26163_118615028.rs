 rust
// Some error that the stage0 compiler doesn't like (E0038 for now)
#[cfg(stage0)]
register_diagnostic! { E0038 }
#[cfg(not(stage0))]
register_diagnostic! { E0038, r##"
Blah blah blah
"##
}
