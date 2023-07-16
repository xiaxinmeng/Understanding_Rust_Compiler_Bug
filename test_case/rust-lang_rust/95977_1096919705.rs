plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0532]: expected tuple struct or tuple variant, found unit variant `ItemKind::KeywordItem`
   --> src/librustdoc/clean/types.rs:499:45
    |
499 |         let visibility = if matches!(&kind, ItemKind::KeywordItem(..) | ItemKind::PrimitiveItem(..))
    |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use this syntax instead: `ItemKind::KeywordItem`
760 |     KeywordItem,
760 |     KeywordItem,
    |     ----------- `ItemKind::KeywordItem` defined here

error[E0532]: expected tuple struct or tuple variant, found unit variant `KeywordItem`
    |
    |
90  |             | KeywordItem(_) => kind,
    |               ^^^^^^^^^^^^^^ help: use this syntax instead: `KeywordItem`
   ::: src/librustdoc/clean/types.rs:760:5
    |
760 |     KeywordItem,
760 |     KeywordItem,
    |     ----------- `KeywordItem` defined here

error[E0532]: expected tuple struct or tuple variant, found unit variant `KeywordItem`
    |
    |
45  |             | KeywordItem(_) => {}
    |               ^^^^^^^^^^^^^^ help: use this syntax instead: `KeywordItem`
   ::: src/librustdoc/clean/types.rs:760:5
    |
760 |     KeywordItem,
760 |     KeywordItem,
    |     ----------- `KeywordItem` defined here
For more information about this error, try `rustc --explain E0532`.
error: could not compile `rustdoc` due to 3 previous errors
Build completed unsuccessfully in 0:03:09
