plain

[TIMING] test::ReplacePlaceholderTest ReplacePlaceholderTest -- 1.527
Building tool rust-analyzer (stage2)
    Updating crates.io index
warning: spurious network error (2 tries remaining): failed to get successful HTTP response from `https://index.crates.io/ru/st/rustc-ap-rustc_lexer`, got 503


warning: spurious network error (1 tries remaining): failed to get successful HTTP response from `https://index.crates.io/ru/st/rustc-ap-rustc_lexer`, got 503

error: failed to get `rustc-ap-rustc_lexer` as a dependency of package `parser v0.0.0 (D:\a\rust\rust\src\tools\rust-analyzer\crates\parser)`

Caused by:
Caused by:
  failed to query replaced source registry `crates-io`

Caused by:
  download of ru/st/rustc-ap-rustc_lexer failed
Caused by:
Caused by:
  failed to get successful HTTP response from `https://index.crates.io/ru/st/rustc-ap-rustc_lexer`, got 503
Build completed unsuccessfully in 1:04:56
Build completed unsuccessfully in 1:04:56
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
