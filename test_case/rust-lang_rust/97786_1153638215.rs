plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0277]: can't compare `PathBuf` with `Option<PathBuf>`
    --> compiler/rustc_metadata/src/rmeta/decoder.rs:1490:70
     |
1490 |             !sess.opts.remap_path_prefix.iter().any(|(_from, to)| to == virtual_dir)
     |                                                                      ^^ no implementation for `PathBuf == Option<PathBuf>`
     |
     = help: the trait `std::cmp::PartialEq<Option<PathBuf>>` is not implemented for `PathBuf`
     = help: the following other types implement trait `std::cmp::PartialEq<Rhs>`:
               <PathBuf as std::cmp::PartialEq<&'a OsStr>>
               <PathBuf as std::cmp::PartialEq<&'a std::path::Path>>
               <PathBuf as std::cmp::PartialEq<Cow<'a, OsStr>>>
               <PathBuf as std::cmp::PartialEq<Cow<'a, std::path::Path>>>
               <PathBuf as std::cmp::PartialEq<OsStr>>
               <PathBuf as std::cmp::PartialEq<OsString>>
               <PathBuf as std::cmp::PartialEq<std::path::Path>>
               <PathBuf as std::cmp::PartialEq>
     = note: required because of the requirements on the impl of `std::cmp::PartialEq<&Option<PathBuf>>` for `&PathBuf`

error[E0277]: `&Filter<Filter<std::array::IntoIter<Option<PathBuf>, 2_usize>, [closure@compiler/rustc_metadata/src/rmeta/decoder.rs:1480:17: 1485:10]>, [closure@compiler/rustc_metadata/src/rmeta/decoder.rs:1486:17: 1491:10]>` is not an iterator
    --> compiler/rustc_metadata/src/rmeta/decoder.rs:1500:32
     |
1500 |             for virtual_dir in &virtual_rust_source_base_dir {
     |                                -^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |                                |
     |                                `&Filter<Filter<std::array::IntoIter<Option<PathBuf>, 2_usize>, [closure@compiler/rustc_metadata/src/rmeta/decoder.rs:1480:17: 1485:10]>, [closure@compiler/rustc_metadata/src/rmeta/decoder.rs:1486:17: 1491:10]>` is not an iterator
     |                                help: consider removing the leading `&`-reference
     |
     = help: the trait `Iterator` is not implemented for `&Filter<Filter<std::array::IntoIter<Option<PathBuf>, 2_usize>, [closure@compiler/rustc_metadata/src/rmeta/decoder.rs:1480:17: 1485:10]>, [closure@compiler/rustc_metadata/src/rmeta/decoder.rs:1486:17: 1491:10]>`
     = note: required because of the requirements on the impl of `IntoIterator` for `&Filter<Filter<std::array::IntoIter<Option<PathBuf>, 2_usize>, [closure@compiler/rustc_metadata/src/rmeta/decoder.rs:1480:17: 1485:10]>, [closure@compiler/rustc_metadata/src/rmeta/decoder.rs:1486:17: 1491:10]>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_metadata` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_metadata` due to 2 previous errors
