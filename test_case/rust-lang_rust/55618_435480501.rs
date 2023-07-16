rust
mod rayon { ... }

mod foo {
    use rayon::join; // today, compiles to extern crate `rayon`
}
