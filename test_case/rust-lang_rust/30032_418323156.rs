ignore` to add compiler error descriptions to my crate docs, which is documented in the book, but as soon as I tried to run `$ rust doc`, it failed because it tried compile the non-Rust characters in that code block as Rust code.

I'm surprised this issue hasn't had any responses since 2016, so either this is already solved and I'm hitting some other issue, or people don't really put non-Rust code blocks in their documentation.

Here's [part of the documentation](https://github.com/rusty-rockets/sm/blob/aeea091476213d9d1c085093403a1cca85da9374/src/lib.rs#L239-L251) that currently fails.