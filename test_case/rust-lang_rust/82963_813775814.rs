plain
   |
56 | use rustc_span::edition::Edition;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`

error[E0616]: field `codes` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:551:23
    |
551 |             cx.shared.codes,

error[E0616]: field `playground` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:553:24
    |
    |
553 |             &cx.shared.playground


error[E0616]: field `codes` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:677:33
    |
677 |     let error_codes = cx.shared.codes;

error[E0616]: field `playground` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:707:28
    |
    |
707 |                 &cx.shared.playground,


error[E0616]: field `issue_tracker_base_url` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:729:55
    |
729 |         if let (Some(url), Some(issue)) = (&cx.shared.issue_tracker_base_url, issue) {

error[E0616]: field `playground` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/mod.rs:749:32
    |
    |
749 |                     &cx.shared.playground,


error[E0616]: field `codes` of struct `context::SharedContext` is private
    --> src/librustdoc/html/render/mod.rs:1415:31
     |
1415 |                     cx.shared.codes,

error[E0616]: field `playground` of struct `context::SharedContext` is private
    --> src/librustdoc/html/render/mod.rs:1417:32
     |
     |
1417 |                     &cx.shared.playground


error[E0616]: field `sort_modules_alphabetically` of struct `context::SharedContext` is private
   --> src/librustdoc/html/render/print_item.rs:185:18
    |
185 |     if cx.shared.sort_modules_alphabetically {

error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0616`.
