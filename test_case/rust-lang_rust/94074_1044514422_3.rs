
Because this is basically what `makeit` does: it expands the default value _including the parentheses of the `#[default(...)]`_.

https://github.com/estebank/makeit/blob/97518801c2ff265b6de226dbaa77f2cfd645a674/makeit-derive/src/lib.rs#L228