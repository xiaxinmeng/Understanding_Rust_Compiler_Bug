
error[E0599]: no method named `to_string` found for struct `html::markdown::MarkdownHtml<'_>` in the current scope
    --> src/librustdoc/html/render.rs:2334:18
     |
2334 |                   .to_string()
     |                    ^^^^^^^^^ help: there is an associated function with a similar name: `into_string`
     | 
    ::: src/librustdoc/html/markdown.rs:73:1
     |
73   | / pub struct MarkdownHtml<'a>(
74   | |     pub &'a str,
75   | |     pub &'a mut IdMap,
76   | |     pub ErrorCodes,
77   | |     pub Edition,
78   | |     pub &'a Option<Playground>,
79   | | );
     | |  -
     | |  |
     | |  method `to_string` not found for this
     | |__doesn't satisfy `_: std::fmt::Display`
     |    doesn't satisfy `_: std::string::ToString`
     |
     = note: the method `to_string` exists but the following trait bounds were not satisfied:
             `html::markdown::MarkdownHtml<'_>: std::fmt::Display`
             which is required by `html::markdown::MarkdownHtml<'_>: std::string::ToString`

error: aborting due to previous error

