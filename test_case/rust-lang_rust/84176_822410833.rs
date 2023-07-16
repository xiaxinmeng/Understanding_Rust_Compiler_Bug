plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find value `BASIC_KEYWORDS` in this scope
   --> src/librustdoc/html/render/context.rs:538:23
    |
538 |             keywords: BASIC_KEYWORDS,
    |
help: consider importing this constant
    |
1   | use crate::html::render::BASIC_KEYWORDS;
1   | use crate::html::render::BASIC_KEYWORDS;
    |

error[E0425]: cannot find value `BASIC_KEYWORDS` in this scope
   --> src/librustdoc/html/render/context.rs:705:23
    |
705 |     format!("{}, {}", BASIC_KEYWORDS, it.name.as_ref().unwrap())
    |
help: consider importing this constant
    |
1   | use crate::html::render::BASIC_KEYWORDS;
