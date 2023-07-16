
error[E0425]: cannot find value `BASIC_KEYWORDS` in this scope
   --> src/librustdoc/html/render/context.rs:538:23
    |
538 |             keywords: BASIC_KEYWORDS,
    |
help: consider importing this constant
    |
1   | use crate::html::render::BASIC_KEYWORDS;
1   | use crate::html::render::BASIC_KEYWORDS;
