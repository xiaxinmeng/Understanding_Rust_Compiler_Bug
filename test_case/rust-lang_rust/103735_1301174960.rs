plain
Successfully built a23692f7640f
Successfully tagged rust-ci:latest
Built container sha256:a23692f7640f2cd5626f9cea88ad42e4399023c91925d2196f64437d72b4d68a
Uploading finished image to https://ci-caches.rust-lang.org/docker/a27a2e8501e6bda8d9ec9572725b52c65accf3f919588efe2ef5cb584fdeae418747b3be4fa090dfcc3ff7ae8714c60234c3f32ed53a403a0831a7e22eb564d1
upload failed: - to s3://rust-lang-ci-sccache2/docker/a27a2e8501e6bda8d9ec9572725b52c65accf3f919588efe2ef5cb584fdeae418747b3be4fa090dfcc3ff7ae8714c60234c3f32ed53a403a0831a7e22eb564d1 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
   Compiling rand v0.7.3
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling core v0.0.0 (/checkout/library/core)
error: implicit auto-ref creates a reference to a dereference of a raw pointer
   |
   |
26 |                     (*c).fetch_add(1, SeqCst);
   |
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
   = note: `-D implicit-unsafe-autorefs` implied by `-D warnings`
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
   |
26 |                     (&(*c)).fetch_add(1, SeqCst);
   |                     ++    +

error: implicit auto-ref creates a reference to a dereference of a raw pointer
    |
    |
105 |         assert_eq!(17, (*p).get());
    |
    = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
    = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
    |
105 |         assert_eq!(17, (&(*p)).get());
    |                        ++    +

error: implicit auto-ref creates a reference to a dereference of a raw pointer
    |
    |
106 |         (*p).set(19);
    |
    = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
    = note: creating a reference requires the pointer to be valid and imposes aliasing requirements
help: try using a raw pointer method instead; or if this reference is intentional, make it explicit
    |
106 |         (&mut (*p)).set(19);

error: could not compile `alloc` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:16:15
