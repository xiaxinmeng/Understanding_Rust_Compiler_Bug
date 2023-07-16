plain
2019-09-01T13:51:24.6886501Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-01T13:51:24.9660434Z error: unused import: `path_local`
2019-09-01T13:51:24.9661309Z  --> src/libsyntax_ext/deriving/cmp/partial_ord.rs:3:23
2019-09-01T13:51:24.9661588Z   |
2019-09-01T13:51:24.9661887Z 3 | use crate::deriving::{path_local, pathvec_std, path_std};
2019-09-01T13:51:24.9662521Z   |
2019-09-01T13:51:24.9662827Z   = note: `-D unused-imports` implied by `-D warnings`
2019-09-01T13:51:24.9683002Z 
2019-09-01T13:51:25.7879312Z error: aborting due to previous error
---
2019-09-01T13:56:09.5310231Z == clock drift check ==
2019-09-01T13:56:09.5326473Z   local time: Sun Sep  1 13:56:09 UTC 2019
2019-09-01T13:56:10.0179824Z   network time: Sun, 01 Sep 2019 13:56:10 GMT
2019-09-01T13:56:10.0182178Z == end clock drift check ==
2019-09-01T13:56:12.6684545Z ##[error]Bash exited with code '1'.
2019-09-01T13:56:12.6717436Z ##[section]Starting: Upload CPU usage statistics
2019-09-01T13:56:12.6720861Z ==============================================================================
2019-09-01T13:56:12.6720930Z Task         : Bash
2019-09-01T13:56:12.6720997Z Description  : Run a Bash script on macOS, Linux, or Windows
