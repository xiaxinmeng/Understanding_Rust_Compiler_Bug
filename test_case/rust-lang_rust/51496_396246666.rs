rust
// Library crate
#[macro_export(macro_helper_hack)]
macro_rules! public {
    ($helper: ident) => {
        $helper!();
    }
}

// User crate
// `my_helper` is not affected by the macro helper hack (not changed to `$crate::my_helper`) and is resolved in the user crate, not library crate
public!(my_helper);
