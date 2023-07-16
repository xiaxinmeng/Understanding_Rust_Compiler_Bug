plain
    Checking chalk-solve v0.76.0
    Checking rustc_log v0.0.0 (/checkout/compiler/rustc_log)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
error[E0599]: the function or associated item `digest` exists for struct `Md5`, but its trait bounds were not satisfied
     |
     |
1314 |                 value.copy_from_slice(&Md5::digest(data));
     |                                             ^^^^^^ function or associated item cannot be called on `Md5` due to unsatisfied trait bounds
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/md-5-0.9.1/src/lib.rs:53:1
     |
53   | pub struct Md5 {
     | --------------
     | --------------
     | |
     | doesn't satisfy `Md5: HashMarker`
     | doesn't satisfy `Md5: sha1::Digest`
     | doesn't satisfy `Md5: sha1::digest::FixedOutput`
     | doesn't satisfy `Md5: sha1::digest::Update`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `Md5: sha1::digest::FixedOutput`
             which is required by `Md5: sha1::Digest`
             `Md5: sha1::digest::Update`
             which is required by `Md5: sha1::Digest`
             `Md5: HashMarker`
             which is required by `Md5: sha1::Digest`
             `&Md5: sha1::digest::FixedOutput`
             which is required by `&Md5: sha1::Digest`
             `&Md5: Default`
             which is required by `&Md5: sha1::Digest`
             `&Md5: sha1::digest::Update`
             which is required by `&Md5: sha1::Digest`
             `&Md5: HashMarker`
             which is required by `&Md5: sha1::Digest`
             `&mut Md5: sha1::digest::FixedOutput`
             which is required by `&mut Md5: sha1::Digest`
             `&mut Md5: Default`
             which is required by `&mut Md5: sha1::Digest`
             `&mut Md5: sha1::digest::Update`
             which is required by `&mut Md5: sha1::Digest`
             `&mut Md5: HashMarker`
             which is required by `&mut Md5: sha1::Digest`
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
     |
32   | use md5::Digest;

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_span` due to previous error
warning: build failed, waiting for other jobs to finish...
