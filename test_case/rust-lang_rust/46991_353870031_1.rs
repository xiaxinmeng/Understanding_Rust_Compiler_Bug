

   Yes, this will mean that "smart" formatting taking extra integer arguments will not be possible. I don't believe anyone is using it in `span_bug!` etc.

   This will require adding `Print` impls to "small" types (e.g. integers).
3. Remove the `Display` impls for `Print` types and make everyone use `cx.wrap`.

Then you could pass the `PrintContext` through `item_path` and move the thread-locals in `item_path` to the `PrintContext`.

Afterward, you could allow a `PrintContext` to keep a reference to a `Diagnostic`, and have it register notes (make sure you replace all the old uses of `PrintContext::new`), so that we could have this error:

If `serde` is accessible in this crate through a cargo dependency and there is no name collision - I think we should use the `[serde]` syntax even while it is not legal, as it is better than just `serde`
