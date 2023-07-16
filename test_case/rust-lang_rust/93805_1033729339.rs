plain
a different dependency.

Run `npm audit` for details.
npm notice 
npm notice New major version of npm available! 7.21.1 -> 8.4.1
npm notice Changelog: <https://github.com/npm/cli/releases/tag/v8.4.1>
npm notice Run `npm install -g npm@8.4.1` to update!
Removing intermediate container 7b306b82bce5
 ---> 72df0aed24e7
Step 6/12 : RUN npm install eslint@8.6.0 -g
 ---> Running in 95cd38f75998
---
Successfully built 13b81cb6a668
Successfully tagged rust-ci:latest
Built container sha256:13b81cb6a668665064de423c868ab0738de9a7ad52ec326bb333845607644db6
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: usage of `ty::TyKind::<kind>`
    |
    |
548 |                     TyKind::Adt(adt_def, _) => {
    |                     ^^^^^^ help: try using ty::<kind> directly: `ty`
    |
    = note: `-D rustc::usage-of-ty-tykind` implied by `-D warnings`

error: usage of `ty::TyKind::<kind>`
    |
    |
556 |                     TyKind::Foreign(def_id) => Res::Def(DefKind::ForeignTy, *def_id),
    |                     ^^^^^^ help: try using ty::<kind> directly: `ty`

error: usage of `ty::TyKind::<kind>`
    |
    |
557 |                     TyKind::Int(int_ty) => Res::Primitive((*int_ty).into()),
    |                     ^^^^^^ help: try using ty::<kind> directly: `ty`

error: usage of `ty::TyKind::<kind>`
    |
    |
558 |                     TyKind::Uint(uint_ty) => Res::Primitive((*uint_ty).into()),
    |                     ^^^^^^ help: try using ty::<kind> directly: `ty`

error: usage of `ty::TyKind::<kind>`
    |
    |
559 |                     TyKind::Float(float_ty) => Res::Primitive((*float_ty).into()),
    |                     ^^^^^^ help: try using ty::<kind> directly: `ty`

error: usage of `ty::TyKind::<kind>`
    |
    |
560 |                     TyKind::Bool => Res::Primitive(PrimitiveType::Bool),
    |                     ^^^^^^ help: try using ty::<kind> directly: `ty`

error: usage of `ty::TyKind::<kind>`
    |
    |
561 |                     TyKind::Char => Res::Primitive(PrimitiveType::Char),
    |                     ^^^^^^ help: try using ty::<kind> directly: `ty`

error: usage of `ty::TyKind::<kind>`
    |
    |
562 |                     TyKind::Str => Res::Primitive(PrimitiveType::Str),
    |                     ^^^^^^ help: try using ty::<kind> directly: `ty`

error: usage of `ty::TyKind::<kind>`
    |
    |
808 |                         TyKind::Adt(adt_def, _) => {
    |                         ^^^^^^ help: try using ty::<kind> directly: `ty`
error: could not compile `rustdoc` due to 9 previous errors
Build completed unsuccessfully in 0:03:42
