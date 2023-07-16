plain
    Checking icu_provider_adapters v1.2.0
error[E0308]: mismatched types
    --> compiler/rustc_span/src/symbol.rs:1986:46
     |
1986 |         if let Some(&name) = inner.names.get(string) {
     |                                          --- ^^^^^^ expected `&*const str`, found `&str`
     |                                          arguments to this method are incorrect
     |
     = note: expected reference `&*const str`
                found reference `&str`
                found reference `&str`
note: method defined here
    --> /checkout/library/std/src/collections/hash/map.rs:876:12
     |
876  |     pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>

   Compiling unic-langid-macros v0.9.1
   Compiling unic-langid v0.9.1
   Compiling intl-memoizer v0.5.1
