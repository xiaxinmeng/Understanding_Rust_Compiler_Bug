rust
macro_rules! expand_to_macro_rules_macro {
    () => {
        #[macro_export]
        macro_rules! quote {
            () => {
                $crate::_quote_impl!()
            };
        }

        #[macro_export]
        macro_rules! _quote_impl {
            () => {
                $crate::quote::foo()
            };
        }
    };
}

#[macro_use]
pub mod quote {
    #[macro_use]
    mod generated {
        expand_to_macro_rules_macro! {}
    }

    pub fn foo() {}
}

fn main() {
    quote!();
}
