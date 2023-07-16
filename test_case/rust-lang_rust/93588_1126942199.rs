bash
git clone https://github.com/diesel-rs/diesel.git && cd diesel # commit 983209a61 for me

# Make all modules public
git ls-files | grep '\.rs' | xargs perl -pi -e 's/^ *mod /pub mod /'

# Undo partially to fix : error: `proc-macro` crate types currently cannot export any items other than functions tagged with `#[proc_macro]`
git restore diesel_derives/src/lib.rs diesel_migrations/migrations_macros/src/lib.rs 

# Remove all #[doc(inline)]
git ls-files | grep '\.rs$' | xargs perl -pi -e 's/.*#\[doc\(inline\)\]//'
