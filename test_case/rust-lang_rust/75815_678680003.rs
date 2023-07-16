
error: `char` is both a module and a builtin type
    --> library/core/src/str/mod.rs:4231:62
     |
4231 |     /// The [pattern] can be a `&str`, [`char`], a slice of [`char`]s, or a
     |                                                              ^^^^^^ ambiguous link
     |
help: to link to the module, prefix with the item type
     |
4231 |     /// The [pattern] can be a `&str`, [`char`], a slice of [`module@char`]s, or a
     |                                                              ^^^^^^^^^^^^^
help: to link to the builtin type, prefix with the item type
     |
4231 |     /// The [pattern] can be a `&str`, [`char`], a slice of [`prim@char`]s, or a
     |                                                              ^^^^^^^^^^^

error: aborting due to 62 previous errors
