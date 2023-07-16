rust
// Not a part of actual macro output, but is instead present at call site. I'm parsing the input struct's type
// specificially (the macro is a derive macro), so it's fine to avoid fully qualifying it, since use super::*;
// will take care of everything.
use std::time::SystemTime; 
mod entries {
    use super::*;
    #[doc = "The entry identifier type for the `field` field in the `MyConfigTable` config table."]
    pub enum Field {}
    impl ::snec::Entry for Field {
        type Data = SystemTime; // This is not in the prelude 
        const NAME: &'static str = "field";
    }
};
...
