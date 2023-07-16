rust
>     // File locking on Unix is currently implemented via `flock`, which is known
>     // to be broken on NFS. We could in theory just ignore errors that happen on
>     // NFS, but apparently the failure mode [1] for `flock` on NFS is **blocking
>     // forever**, even if the "non-blocking" flag is passed!
>     //
>     // As a result, we just skip all file locks entirely on NFS mounts. That
>     // should avoid calling any `flock` functions at all, and it wouldn't work
>     // there anyway.
>     //
>     // [1]: https://github.com/rust-lang/cargo/issues/2615
> 