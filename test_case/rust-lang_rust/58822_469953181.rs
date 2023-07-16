
error: use of deprecated item 'hash::InsecureSha1': SHA-1 is considered insecure
   --> src/hash.rs:102:10
    |
102 | #[derive(Default)]
    |          ^^^^^^^
    |
note: lint level defined here
   --> src/lib.rs:45:9
    |
45  | #![deny(warnings)]
    |         ^^^^^^^^
    = note: #[deny(deprecated)] implied by #[deny(warnings)]

error: use of deprecated item 'hash::InsecureSha1': SHA-1 is considered insecure
   --> src/hash.rs:102:10
    |
102 | #[derive(Default)]
    |          ^^^^^^^

error: use of deprecated item 'hash::InsecureSha1': SHA-1 is considered insecure
   --> src/hash.rs:102:10
    |
102 | #[derive(Default)]
    |          ^^^^^^^
    |
note: lint level defined here
   --> src/lib.rs:45:9
    |
45  | #![deny(warnings)]
    |         ^^^^^^^^
    = note: #[deny(deprecated)] implied by #[deny(warnings)]

error: use of deprecated item 'hash::InsecureSha1': SHA-1 is considered insecure
   --> src/hash.rs:102:10
    |
102 | #[derive(Default)]
    |          ^^^^^^^

error: use of deprecated item 'hash::InsecureSha1::ctx': SHA-1 is considered insecure
   --> src/hash.rs:104:5
    |
104 |     ctx: CStackWrapper<boringssl::SHA_CTX>,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
