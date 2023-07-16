rust
> fn sort(blobs: &mut Vec<(String, Vec<u8>)>) {
>     blobs.sort_unstable_by_key(|blob| &blob.0);
> }
> 