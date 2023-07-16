

error: unresolved link to `Formatter::debug_map`
   --> library/core/src/fmt/builders.rs:692:37
    |
692 | /// This can be constructed by the [`Formatter::debug_map`] method.
    |                                     ^^^^^^^^^^^^^^^^^^^^^^ the module `builders` contains no item named `Formatter`

error: unresolved link to `Debug::fmt`
   --> library/core/src/fmt/builders.rs:430:14
    |
430 | /// of your [`Debug::fmt`] implementation.
    |              ^^^^^^^^^^^^ `Debug` is a derive macro, not a module or type, and cannot have associated items
