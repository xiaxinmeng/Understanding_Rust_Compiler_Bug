console
$ ./x.py build --stage 1
error[E0308]: mismatched types
   --> src/librustdoc/html/markdown.rs:942:72
    |
934 |           let replacer = |_: &str, s: &str| {
    |  ________________________-
935 | |             if let Some(link) = links.iter().find(|link| &*link.original_text == s) {
936 | |                 Some((link.href.clone(), link.new_text.clone()))
937 | |             } else {
938 | |                 None
939 | |             }
940 | |         };
    | |_________- the found closure
941 | 
942 |           let p = Parser::new_with_broken_link_callback(md, opts(), Some(&replacer));
    |                                                                          ^^^^^^^^^ types differ in mutability
    |
    = note: expected mutable reference `&mut dyn for<'r> std::ops::FnMut(pulldown_cmark::BrokenLink<'r>) -> std::option::Option<(pulldown_cmark::CowStr<'_>, pulldown_cmark::CowStr<'_>)>`
                       found reference `&[closure@src/librustdoc/html/markdown.rs:934:24: 940:10 links:_]`

error[E0308]: mismatched types
    --> src/librustdoc/html/markdown.rs:1023:18
     |
1012 |           let replacer = |_: &str, s: &str| {
     |  ________________________-
1013 | |             if let Some(link) = links.iter().find(|link| &*link.original_text == s) {
1014 | |                 Some((link.href.clone(), link.new_text.clone()))
1015 | |             } else {
1016 | |                 None
1017 | |             }
1018 | |         };
     | |_________- the found closure
...
1023 |               Some(&replacer),
     |                    ^^^^^^^^^ types differ in mutability
     |
     = note: expected mutable reference `&mut dyn for<'r> std::ops::FnMut(pulldown_cmark::BrokenLink<'r>) -> std::option::Option<(pulldown_cmark::CowStr<'_>, pulldown_cmark::CowStr<'_>)>`
                        found reference `&[closure@src/librustdoc/html/markdown.rs:1012:24: 1018:10 links:_]`

error[E0308]: mismatched types
    --> src/librustdoc/html/markdown.rs:1091:72
     |
1087 |           let push = |_: &str, s: &str| {
     |  ____________________-
1088 | |             shortcut_links.borrow_mut().push((s.to_owned(), locate(s)));
1089 | |             None
1090 | |         };
     | |_________- the found closure
1091 |           let p = Parser::new_with_broken_link_callback(md, opts(), Some(&push));
     |                                                                          ^^^^^ types differ in mutability
     |
     = note: expected mutable reference `&mut dyn for<'r> std::ops::FnMut(pulldown_cmark::BrokenLink<'r>) -> std::option::Option<(pulldown_cmark::CowStr<'_>, pulldown_cmark::CowStr<'_>)>`
                        found reference `&[closure@src/librustdoc/html/markdown.rs:1087:20: 1090:10 shortcut_links:_, locate:_]`

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc`.
