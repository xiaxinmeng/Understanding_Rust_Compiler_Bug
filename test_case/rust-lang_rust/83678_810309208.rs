
error: Found non-existing keyword `SelfTy` used in `#[doc(keyword = "...")]`
    --> library/std/src/keyword_docs.rs:1316:1
     |
1316 | #[doc(keyword = "SelfTy")]
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: the lint level is defined here
    --> library/std/src/lib.rs:216:9
     |
216  | #![deny(rustc::existing_doc_keyword)]
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: only existing keywords are allowed in core/std
