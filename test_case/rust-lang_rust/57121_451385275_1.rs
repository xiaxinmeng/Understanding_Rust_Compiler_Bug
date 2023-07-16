
glaubitz@epyc:..rust/rust> cargo update -p libc
    Updating crates.io index
error: no matching package named `core` found                                                                                                                                                
location searched: registry `https://github.com/rust-lang/crates.io-index`
required by package `backtrace-sys v0.1.27`
    ... which is depended on by `backtrace v0.3.11`
    ... which is depended on by `error-chain v0.12.0`
    ... which is depended on by `cargo_metadata v0.6.2`
    ... which is depended on by `miri v0.1.0 (/local_scratch/glaubitz/rust/rust/src/tools/miri)`
glaubitz@epyc:..rust/rust>
