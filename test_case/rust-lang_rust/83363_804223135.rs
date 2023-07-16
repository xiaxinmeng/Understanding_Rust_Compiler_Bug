rust
#[cfg(bootstrap)]
use syn::spanned::MultiSpan;
#[cfg(not(bootstrap))]
use syn::spanned::Spanned;
