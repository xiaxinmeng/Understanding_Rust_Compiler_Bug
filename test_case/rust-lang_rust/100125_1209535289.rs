
==================================================================================
= Please file this regression report on the rust-lang/rust GitHub repository     =
=        New issue: https://github.com/rust-lang/rust/issues/new                 =
=     Known issues: https://github.com/rust-lang/rust/issues                     =
= Copy and paste the text below into the issue report thread.  Thanks!           =
==================================================================================

searched nightlies: from nightly-2022-05-16 to nightly-2022-05-20
regressed nightly: nightly-2022-05-19
searched commit range: https://github.com/rust-lang/rust/compare/4c5f6e6277b89e47d73a192078697f7a5f3dc0ac...cd282d7f75da9080fda0f1740a729516e7fbec68
regressed commit: https://github.com/rust-lang/rust/commit/936eba3b348e65b658b60218cc9237f02abdbeb4

<details>
<summary>bisected with <a href='https://github.com/rust-lang/cargo-bisect-rustc'>cargo-bisect-rustc</a> v0.6.4</summary>


Host triple: x86_64-unknown-linux-gnu
Reproduce with:

cargo bisect-rustc ./script.sh --start 2022-05-16 --end 2022-05-20 --preserve 

</details>
